/* core/backend/appstream.rs
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

use appstream::{ffi::as_utils_sort_components_into_categories, Category, Component};
use glib::translate::{IntoGlib, ToGlibPtr};

pub fn sort_components_into_categories(
    cpts: &[Component],
    categories: &[Category],
    check_duplicates: bool,
) {
    unsafe {
        as_utils_sort_components_into_categories(
            cpts.to_glib_none().0,
            categories.to_glib_none().0,
            check_duplicates.into_glib(),
        );
    }
}
