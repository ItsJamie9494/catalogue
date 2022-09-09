/* core/package.rs
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
use adw::subclass::prelude::*;
use appstream::prelude::*;
use appstream::Component;
use appstream::IconKind;
use appstream::Release;
use gio::File;
use gio::FileIcon;
use gio::Icon;
use gio::ThemedIcon;
use gtk::IconTheme;
use gtk::{
    gdk::Display,
    glib::{self, Object},
    prelude::*,
};
use std::cmp::Ordering;

mod imp {
    use std::cell::RefCell;

    use glib::{ParamSpec, ParamSpecObject, ParamSpecString, Value};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Package {
        pub component: RefCell<Component>,

        pub name: RefCell<Option<String>>,
        pub version: RefCell<Option<String>>,
        pub summary: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Package {
        const NAME: &'static str = "Package";
        type Type = super::Package;
        type ParentType = Object;
    }

    impl ObjectImpl for Package {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecObject::builder("component", Component::static_type()).build(),
                    ParamSpecString::builder("name").build(),
                    ParamSpecString::builder("version").build(),
                    ParamSpecString::builder("summary").build(),
                    ParamSpecObject::builder("icon", Icon::static_type()).build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "component" => {
                    self.component.replace(
                        value
                            .get::<Component>()
                            .expect("The value needs to be of type `AsComponent`"),
                    );
                }
                "name" => {
                    self.name.replace(Some(
                        value
                            .get::<String>()
                            .expect("The value needs to be of type `String`"),
                    ));
                },
                "version" => {
                    self.version.replace(Some(
                        value
                            .get::<String>()
                            .expect("The value needs to be of type `String`"),
                    ));
                }
                "summary" => {
                    self.summary.replace(Some(
                        value
                            .get::<String>()
                            .expect("The value needs to be of type `String`"),
                    ));
                }
                "icon" => {
                    println!("Unimplemented but should not fail");
                    return;
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "component" => self.component.borrow().to_value(),
                "name" => obj.name().to_value(),
                "version" => obj.version().to_value(),
                "summary" => obj.summary().to_value(),
                // For more precise measurements, just call the function directly
                "icon" => obj.icon(64, 64).to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct Package(ObjectSubclass<imp::Package>);
}

impl Package {
    pub fn new(component: Component) -> Self {
        Object::new(&[("component", &component)]).expect("Failed to create Package")
    }

    pub fn component(&self) -> Component {
        self.imp().component.borrow().clone()
    }

    pub fn set_component(&self, component: Component) {
        self.imp().name.replace(None);
        self.imp().summary.replace(None);
        self.imp().component.replace(component);
    }

    pub fn name(&self) -> String {
        let name_ref = self.imp().name.borrow();
        if name_ref.is_some() {
            return name_ref.clone().unwrap();
        } else {
            let name = self.imp().component.borrow().name().map(|x| x.to_string());
            if name.is_none() {
                todo!();
            }

            return name.unwrap();
        }
    }

    // TODO write this function. Need to check AppStream metadata and probably Flatpak ref details.
    // Maybe add a PackageDetails trait that backends can use
    pub fn version(&self) -> String {
        return String::from("Nya!");
    }

    pub fn summary(&self) -> String {
        let summary_ref = self.imp().summary.borrow();
        if summary_ref.is_some() {
            return summary_ref.clone().unwrap();
        } else {
            let summary = self
                .imp()
                .component
                .borrow()
                .summary()
                .map(|x| x.to_string());
            if summary.is_none() {
                todo!();
            }

            return summary.unwrap();
        }
    }

    /// By default, when using Properties, this function will return a 64x64 icon
    /// Call this function manually to change the scale or size
    pub fn icon(&self, size: u32, scale: u32) -> Icon {
        let mut icon: Icon = ThemedIcon::new("application-default-icon").upcast::<Icon>();
        let pixel_size = size * scale;

        let mut current_size = 0;
        let mut current_scale = 0;

        let icons = self.imp().component.borrow().icons();
        for _icon in icons.iter() {
            let icon_scale = _icon.scale();
            let icon_width = _icon.width() * icon_scale;
            let is_icon_bigger = icon_width > current_size && current_size < pixel_size;
            let icon_has_better_dpi =
                icon_width == current_size && current_scale < icon_scale && scale <= icon_scale;

            match _icon.kind() {
                IconKind::Unknown => {
                    break;
                }
                IconKind::Cached | IconKind::Local => {
                    if is_icon_bigger || icon_has_better_dpi {
                        let icon_file = File::for_path(_icon.filename().unwrap());
                        icon = FileIcon::new(&icon_file).upcast::<Icon>();
                        current_scale = icon_scale;
                        current_size = icon_width;
                    }
                }
                IconKind::Stock => {
                    let name = _icon.name().unwrap();
                    if IconTheme::for_display(&Display::default().unwrap()).has_icon(&name) {
                        icon = ThemedIcon::new(&name).upcast::<Icon>();
                    }
                }
                IconKind::Remote => {
                    if is_icon_bigger || icon_has_better_dpi {
                        let icon_file = File::for_uri(&_icon.url().unwrap());
                        icon = FileIcon::new(&icon_file).upcast::<Icon>();
                        current_scale = icon_scale;
                        current_size = icon_width;
                    }
                }
                _ => break,
            };
        }

        icon
    }

    // Max releases is needed for determining Installed, but since we don't check that yet,
    // it's not needed
    pub fn get_newest_releases (&self, min_releases: usize, _max_releases: usize) -> Vec<Release> {
        let mut list: Vec<Release> = Vec::new();
        let mut releases = self.imp().component.borrow().releases();

        for rel in releases.clone().iter() {
            if rel.version().is_none() {
                releases.remove(releases.iter().position(|x| x == rel).expect("Expected a Release"));
            }
        }

        if releases.len() < min_releases {
            return list;
        }

        releases.sort_by(|a, b| {
            match b.vercmp(a) {
                -1 => Ordering::Less,
                0 => Ordering::Equal,
                1 => Ordering::Greater,
                // This function should only return -1, 0, or 1.
                _ => unimplemented!("Invalid Release Comparison"),
            }
        });

        // TODO Add checks for Installed apps

        for rel in 0..min_releases {
            // Clone is required again, no Copy
            list.insert(rel, releases.get(rel).expect("Expected a release").clone());
        }

        list
    }

    pub fn get_latest_release (&self) -> Option<Release> {
        let mut releases = self.imp().component.borrow().releases();
        releases.sort_by(|a, b| {
            if a.version().is_none() || b.version().is_none() {
                a.version().map(|_| return Ordering::Less);
                b.version().map(|_| return Ordering::Greater);
                return Ordering::Equal;
            } else {
                match b.vercmp(a) {
                    -1 => Ordering::Less,
                    0 => Ordering::Equal,
                    1 => Ordering::Greater,
                    // This function should only return -1, 0, or 1.
                    _ => unimplemented!("Invalid Release Comparison"),
                }
            }
        });

        // Using x.clone() to dereference, as Release doesn't support Copy or *
        releases.get(0).map(|x| x.clone())
    }
}

impl Default for Package {
    fn default() -> Self {
        Object::new(&[]).expect("Failed to create Package")
    }
}
