/* core/backend/mod.rs
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

use super::package::Package;
use ::appstream::Category;
use dyn_clone::DynClone;

pub mod appstream;
pub mod flatpak;

pub trait Backend: DynClone {
    fn get_package_for_component_id(&self, id: String) -> Option<Package>;
    fn get_packages_for_category(&self, category: Category) -> Vec<Package>;
    fn get_recently_updated_packages(&self, size: usize) -> Vec<Package>;
    fn get_installed_packages(&self) -> Vec<Package>;
    fn refresh_cache(&self);
}
