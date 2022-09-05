/* core/utils.rs
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

use std::{
    error::Error,
    fs::{metadata, read_dir, remove_file},
    path::Path,
    time::SystemTime,
};

/// Remove all contents of a directory, without removing the directory itself
pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(path)? {
        remove_file(entry?.path())?;
    }
    Ok(())
}

/// Returns the file age as a u64
pub fn get_file_age<P: AsRef<Path>>(path: P) -> Result<u64, Box<dyn Error>> {
    let metadata = metadata(path)?;

    if let Ok(time) = metadata.modified() {
        let now = SystemTime::now();

        return Ok(now.duration_since(time)?.as_secs());
    } else {
        println!("Not supported");
    }

    Ok(u64::MAX)
}
