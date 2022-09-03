/* application.rs
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
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
};

use crate::action;
use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::CatalogueWindow;
use log::{debug, info};

mod imp {
    use crate::widgets::carousel::{Carousel, CarouselTile};

    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct CatalogueApplication {
        pub window: OnceCell<WeakRef<CatalogueWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CatalogueApplication {
        const NAME: &'static str = "CatalogueApplication";
        type Type = super::CatalogueApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for CatalogueApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            Carousel::ensure_type();
            CarouselTile::ensure_type();
        }
    }
    impl ApplicationImpl for CatalogueApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("AdwApplication<CatalogueApplication>::activate");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = CatalogueWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<ExampleApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for CatalogueApplication {}
    impl AdwApplicationImpl for CatalogueApplication {}
}

glib::wrapper! {
    pub struct CatalogueApplication(ObjectSubclass<imp::CatalogueApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl CatalogueApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &"/dev/itsjamie/Catalogue"),
        ])
        .expect("Failed to create CatalogueApplication")
    }

    fn main_window(&self) -> CatalogueWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        action!(
            self,
            "about",
            clone!(@weak self as app => move |_, _| {
                app.show_about();
            })
        );
        action!(
            self,
            "quit",
            clone!(@weak self as app => move |_, _| {
                app.quit()
            })
        );
    }

    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn show_about(&self) {
        let mut name = "Catalogue".to_owned();

        if PROFILE == "Devel" {
            name.push_str(" Devel");
        }

        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name(&name)
            .application_icon(APP_ID)
            .developer_name("Jamie Murphy")
            .version(VERSION)
            .developers(vec!["Jamie Murphy".into()])
            .website("https://github.com/ItsJamie9494/catalogue")
            .issue_url("https://github.com/ItsJamie9494/catalogue/issues")
            .license_type(gtk::License::Gpl30)
            .build();

        about.present();
    }

    pub fn run(&self) {
        info!("Catalogue ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
