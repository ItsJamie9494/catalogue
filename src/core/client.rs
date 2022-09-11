/* core/client.rs
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

use log::{debug, warn};

use super::{
    backend::{flatpak::FlatpakBackend, Backend},
    package::Package,
};

dyn_clone::clone_trait_object!(Backend);

#[derive(Clone)]
pub struct Client {
    active_backend: Box<dyn Backend>,
}

impl Client {
    pub fn get_package_for_component_id(&self, id: String) -> Option<Package> {
        self.active_backend.get_package_for_component_id(id)
    }

    /// Asyncronously refresh the current backend
    pub async fn refresh_cache(&self, force_update: bool) {
        debug!("Updating Cache");

        // TODO Handle Refresh Timeouts

        if force_update {
            if online::check(None).await.is_ok() {
                self.active_backend.refresh_cache();
            } else {
                warn!("No Internet Connection");
            }
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            active_backend: Box::new(FlatpakBackend::default()),
        }
    }
}
