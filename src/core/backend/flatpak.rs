/* core/backend/flatpak.rs
 *
 * Copyright 2022 Jamie Murphy
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{
    config::APP_ID,
    core::{
        package::Package,
        utils::{get_file_age, remove_dir_contents, xml::fixup},
    },
};
use appstream::{prelude::*, BundleKind, FormatStyle, Pool, PoolFlags};
use dirs::cache_dir;
use flatpak::{prelude::*, Installation, Remote};
use gio::{traits::FileExt, Cancellable, FileMonitor};
use log::{debug, warn};
use std::{
    cell::RefCell, collections::HashMap, error::Error, fs::create_dir_all, os::unix::fs::symlink,
    path::PathBuf,
};

use super::Backend;

pub struct FlatpakBackend {
    package_list: RefCell<HashMap<String, Package>>,
    user_pool: Pool,
    system_pool: Pool,
    user_metadata: String,
    system_metadata: String,

    cancellable: Cancellable,
    pub user_installation: Option<Installation>,
    pub system_installation: Option<Installation>,
    pub user_installation_monitor: Option<FileMonitor>,
    pub system_installation_monitor: Option<FileMonitor>,
}

impl Backend for FlatpakBackend {
    fn get_package_for_component_id(&self, id: String) -> Option<Package> {
        let suffixed_id = format!("{}.desktop", id);

        if !self.package_list.borrow().is_empty() {
            for package in self.package_list.borrow().values().into_iter() {
                if package
                    .component()
                    .id()
                    .map(|x| x.to_string() == id)
                    .expect("Expected an ID")
                {
                    return Some(package.clone());
                } else if package
                    .component()
                    .id()
                    .map(|x| x.to_string() == suffixed_id)
                    .expect("Expected an ID")
                {
                    return Some(package.clone());
                } else {
                    continue;
                }
            }
            return None;
        } else {
            return None;
        }
    }

    fn refresh_cache(&self) {
        let mut remotes: Vec<Remote> = Vec::new();

        if self.user_installation.is_none() && self.system_installation.is_none() {
            warn!("No flatpak installations");
            return;
        }

        if self.user_installation.is_some() {
            self.user_installation
                .clone()
                .unwrap()
                .drop_caches(Some(&self.cancellable))
                .expect("Failed to drop current Flatpak cache");
            remotes.clear();
            remotes.append(
                &mut self
                    .user_installation
                    .clone()
                    .unwrap()
                    .list_remotes(Some(&self.cancellable))
                    .expect("Failed to get Flatpak remotes from user installation"),
            );
            self.preprocess_appstream_metadata(false, &remotes);

            self.reload_appstream_pool(&self.user_pool, &self.user_metadata)
                .unwrap();
        }

        if self.system_installation.is_some() {
            self.system_installation
                .clone()
                .unwrap()
                .drop_caches(Some(&self.cancellable))
                .expect("Failed to drop current Flatpak cache");
            remotes.clear();
            remotes.append(
                &mut self
                    .system_installation
                    .clone()
                    .unwrap()
                    .list_remotes(Some(&self.cancellable))
                    .expect("Failed to get Flatpak remotes from system installation"),
            );
            self.preprocess_appstream_metadata(true, &remotes);

            self.reload_appstream_pool(&self.system_pool, &self.system_metadata)
                .unwrap();
        }
    }
}

impl FlatpakBackend {
    fn reload_appstream_pool(&self, pool: &Pool, metadata: &String) -> Result<(), Box<dyn Error>> {
        pool.reset_extra_data_locations();
        pool.add_extra_data_location(&metadata, FormatStyle::Collection);

        debug!("Loading Pool...");
        pool.load(Some(&self.cancellable))?;
        for comp in pool.components().iter() {
            let bundle = comp.bundle(BundleKind::Flatpak);
            match bundle {
                Some(bundle) => {
                    let key = Self::generate_package_list_key(
                        false,
                        comp.origin()
                            .map(|x| x.to_string())
                            .expect("Expected a string"),
                        bundle
                            .id()
                            .map(|x| x.to_string())
                            .expect("Expected a string"),
                    );

                    let mut pkg_list = self.package_list.borrow_mut();
                    let package = pkg_list.get_key_value(&key);

                    match package {
                        Some(package) => {
                            package.1.set_component(comp.clone());
                        }
                        None => {
                            pkg_list.insert(key, Package::new(comp.clone()));
                        }
                    }
                }
                None => {
                    warn!("Failed to find bundle with ID {:?}", comp.id());
                }
            }
        }
        Ok(())
    }

    fn preprocess_appstream_metadata(&self, system: bool, remotes: &Vec<Remote>) {
        let dest_path: String;
        let installation: Option<Installation>;

        if system {
            dest_path = self.system_metadata.clone();
            installation = self.system_installation.clone();
        } else {
            dest_path = self.user_metadata.clone();
            installation = self.user_installation.clone();
        }

        if installation.is_none() {
            return;
        } else {
            create_dir_all(&dest_path).expect("Failed to create Flatpak metadata directory");

            remove_dir_contents(&dest_path).unwrap();

            for remote in remotes.iter() {
                let mut refresh_needed = true;
                let origin_name = remote.name().map(|x| x.to_string()).unwrap();
                debug!("Found remote {}", origin_name);

                if remote.is_disabled() {
                    debug!("{} is disabled, skipping", origin_name);
                    continue;
                }

                let timestamp = remote.appstream_timestamp(None).unwrap();
                if !timestamp.query_exists(Some(&self.cancellable)) {
                    // Refresh
                    refresh_needed = true;
                } else {
                    let age = get_file_age(timestamp.path().unwrap()).unwrap();
                    debug!("Age: {}", age);

                    // File should be at least 1 hour old before refreshing
                    if age > 3600 {
                        refresh_needed = true;
                    }
                }

                if refresh_needed {
                    debug!("Updating remote metadata");
                    installation
                        .clone()
                        .unwrap()
                        .update_remote_sync(&origin_name, Some(&self.cancellable))
                        .expect("Failed to update remote");

                    debug!("Updating remote appstream metadata");
                    installation
                        .clone()
                        .unwrap()
                        .update_appstream_sync(&origin_name, None, Some(&self.cancellable))
                        .expect("Failed to update appstream");

                    let mut metadata_file =
                        PathBuf::from(remote.appstream_dir(None).unwrap().path().unwrap());
                    metadata_file.push("appstream.xml");
                    let mut metadata_dest = PathBuf::from(&dest_path);
                    metadata_dest.push(format!("{}.xml", &origin_name));

                    if metadata_file.exists() {
                        fixup(&origin_name, metadata_file, metadata_dest).unwrap();

                        let mut local_icons_path = PathBuf::from(&dest_path);
                        local_icons_path.push("icons");

                        if !local_icons_path.exists() {
                            create_dir_all(&local_icons_path).expect(
                                "Error creating Flatpak icon structure, icons may not display",
                            );
                        }

                        let mut remote_icons_path =
                            PathBuf::from(remote.appstream_dir(None).unwrap().path().unwrap());
                        remote_icons_path.push("icons");
                        if !remote_icons_path.exists() {
                            debug!("Remote icons missing for remote {}", origin_name);
                            continue;
                        }

                        remote_icons_path.push(&origin_name);
                        local_icons_path.push(&origin_name);
                        if remote_icons_path.exists() {
                            symlink(&remote_icons_path, &local_icons_path).expect(
                                "Error creating Flatpak icon structure, icons may not display",
                            );
                        } else {
                            remote_icons_path.pop();
                            symlink(&remote_icons_path, &local_icons_path).expect(
                                "Error creating Flatpak icon structure, icons may not display",
                            );
                        }
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    fn generate_package_list_key(system: bool, origin: String, bundle_id: String) -> String {
        let installation = system.then(|| return String::from("system"));
        return format!(
            "{}/{}/{}",
            installation.unwrap_or(String::from("user")),
            origin,
            bundle_id
        );
    }
}

impl Default for FlatpakBackend {
    fn default() -> Self {
        let cancellable = Cancellable::new();
        let user_installation = Installation::new_user(Some(&cancellable)).ok();
        let system_installation = Installation::new_system(Some(&cancellable)).ok();

        // TODO actually do things with these fuckers
        let user_installation_monitor = user_installation
            .clone()
            .map(|x| x.create_monitor(Some(&cancellable)));
        let system_installation_monitor = system_installation
            .clone()
            .map(|x| x.create_monitor(Some(&cancellable)));

        // Pools
        let user_pool = Pool::new();
        user_pool.set_flags(PoolFlags::LOAD_OS_COLLECTION);
        let system_pool = Pool::new();
        system_pool.set_flags(PoolFlags::LOAD_OS_COLLECTION);
        let mut user_metadata = PathBuf::new();
        user_metadata.push(cache_dir().unwrap());
        user_metadata.push(APP_ID);
        user_metadata.push("flatpak-metadata");
        user_metadata.push("user");
        let mut system_metadata = PathBuf::new();
        system_metadata.push(cache_dir().unwrap());
        system_metadata.push(APP_ID);
        system_metadata.push("flatpak-metadata");
        system_metadata.push("system");

        Self {
            package_list: RefCell::new(HashMap::new()),
            user_pool,
            system_pool,
            user_metadata: String::from(user_metadata.to_str().unwrap()),
            system_metadata: String::from(system_metadata.to_str().unwrap()),
            cancellable,
            user_installation,
            system_installation,
            user_installation_monitor: user_installation_monitor.map(|x| x.ok()).unwrap(),
            system_installation_monitor: system_installation_monitor.map(|x| x.ok()).unwrap(),
        }
    }
}
