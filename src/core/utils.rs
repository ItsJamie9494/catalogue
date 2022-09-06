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
    fs::{metadata, read_dir, remove_dir, remove_file},
    path::Path,
    time::SystemTime,
};

use log::error;

/// Remove all contents of a directory, without removing the directory itself
pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            remove_dir_contents(&path)?;
            remove_dir(path)?;
        } else {
            remove_file(path)?;
        }
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
        error!("Not supported");
    }

    Ok(u64::MAX)
}

pub mod xml {
    use std::{
        error::Error,
        fs::File,
        io::{prelude::*, BufRead, BufReader, LineWriter},
        path::PathBuf,
    };

    pub fn fixup(name: &String, source: PathBuf, dest: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut reader = BufReader::new(File::open(source)?);
        let mut buf = vec![];
        let mut writer = LineWriter::new(File::create(dest)?);

        while let Ok(_) = reader.read_until(b'\n', &mut buf) {
            if buf.is_empty() {
                break;
            }
            let line = String::from_utf8_lossy(&buf);
            let mut bytes = line.to_string();

            if line.contains("<components version=\"0.8\" origin=\"flatpak\">") {
                bytes = line.replace("flatpak", name);
            }
            writer.write_all(bytes.as_bytes())?;
            buf.clear();
        }

        Ok(())
    }
}
