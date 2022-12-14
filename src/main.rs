/* main.rs
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
#![deny(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::unused_self)]
#![allow(clippy::module_name_repetitions)]

extern crate pretty_env_logger;

mod application;
#[rustfmt::skip]
mod config;
mod core;
mod macros;
mod widgets;
mod window;

use std::env;

use self::application::CatalogueApplication;
use self::window::CatalogueWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, gettext, textdomain};
use gtk::{gio, glib};
use log::error;

fn main() {
    pretty_env_logger::init();

    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Catalogue"));

    let resources = match env::var("MESON_DEVENV") {
        Err(_) => gio::Resource::load(RESOURCES_FILE).expect("Could not load resources"),
        Ok(_) => match env::current_exe() {
            Ok(path) => {
                let mut resource_path = path;
                resource_path.pop();
                resource_path.pop();
                resource_path.push("data");
                resource_path.push("resources.gresource");
                gio::Resource::load(&resource_path)
                    .expect("Unable to find resources.gresource in devenv")
            }
            Err(err) => {
                error!("Unable to find the current path: {}", err);
                return;
            }
        },
    };
    gio::resources_register(&resources);

    let app = CatalogueApplication::new();
    app.run();
}
