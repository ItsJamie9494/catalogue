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

use crate::core::package::Package;
use appstream::prelude::*;
use std::collections::HashMap;

use super::Backend;

pub struct FlatpakBackend {
    package_list: HashMap<String, Package>,
}

impl Backend for FlatpakBackend {
    fn get_package_for_component_id(&self, id: String) -> Option<Package> {
        let suffixed_id = format!("{}.desktop", id);

        if !self.package_list.is_empty() {
            for package in self.package_list.values().into_iter() {
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
                    return None;
                }
            }
            return None;
        } else {
            return None;
        }
    }
}

impl Default for FlatpakBackend {
    fn default() -> Self {
        // INSERT DEFAULT OR CONSTRUCT VALUES
        Self {
            package_list: HashMap::new(),
        }
    }
}
