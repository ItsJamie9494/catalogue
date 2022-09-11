/* window.rs
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
use adw::NavigationDirection;
use appstream::prelude::*;
use appstream::Category;
use gtk::{gio, glib, prelude::*, CompositeTemplate};

use crate::application::CatalogueApplication;
use crate::config::{APP_ID, PROFILE};
use crate::widgets::app_tile::AppTile;
use crate::widgets::category_page::CategoryPage;
use crate::widgets::category_tile::CategoryTile;

mod imp {
    use adw::{Leaflet, WindowTitle};
    use gtk::{gio::Settings, template_callbacks, Box, Button, FlowBox};

    use crate::core::category::CatalogueCategories;

    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/dev/itsjamie/Catalogue/window.ui")]
    pub struct CatalogueWindow {
        #[template_child]
        pub category_box: TemplateChild<FlowBox>,

        #[template_child]
        pub recent_box: TemplateChild<FlowBox>,

        #[template_child]
        pub subpage_leaflet: TemplateChild<Leaflet>,

        #[template_child]
        pub subpage_title: TemplateChild<WindowTitle>,

        #[template_child]
        pub subpage_content: TemplateChild<Box>,

        pub settings: Settings,
    }

    #[template_callbacks]
    impl CatalogueWindow {
        #[template_callback]
        fn leaflet_back_clicked_cb(&self, _button: &Button) {
            self.subpage_leaflet.navigate(NavigationDirection::Back);
        }
    }

    impl Default for CatalogueWindow {
        fn default() -> Self {
            Self {
                category_box: TemplateChild::default(),
                recent_box: TemplateChild::default(),
                subpage_leaflet: TemplateChild::default(),
                subpage_title: TemplateChild::default(),
                subpage_content: TemplateChild::default(),
                settings: Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CatalogueWindow {
        const NAME: &'static str = "CatalogueWindow";
        type Type = super::CatalogueWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CatalogueWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            obj.load_category_tile(CatalogueCategories::default().create);
            obj.load_category_tile(CatalogueCategories::default().work);
            obj.load_category_tile(CatalogueCategories::default().games);
            obj.load_category_tile(CatalogueCategories::default().internet);
            obj.load_category_tile(CatalogueCategories::default().develop);
            obj.load_category_tile(CatalogueCategories::default().accessories);

            obj.load_recent_box();
            obj.load_window_size();
        }
    }
    impl WidgetImpl for CatalogueWindow {}
    impl WindowImpl for CatalogueWindow {
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            self.parent_close_request(window)
        }
    }
    impl ApplicationWindowImpl for CatalogueWindow {}
    impl AdwApplicationWindowImpl for CatalogueWindow {}
}

glib::wrapper! {
    pub struct CatalogueWindow(ObjectSubclass<imp::CatalogueWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl CatalogueWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create CatalogueWindow")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    fn load_category_tile(&self, category: Category) {
        let btn = CategoryTile::new(category);

        let content = self.imp().subpage_content.clone();
        let leaflet = self.imp().subpage_leaflet.clone();
        let title = self.imp().subpage_title.clone();
        btn.connect_clicked(move |tile| {
            let category = tile.category();
            // Remove previous page
            if !content
                .last_child()
                .unwrap()
                .widget_name()
                .to_string()
                .contains("HeaderBar")
            {
                content.last_child().unwrap().unparent();
            }

            title.set_title(&category.name().expect("Expected a string"));
            content.append(&CategoryPage::new(tile.category()));

            leaflet.navigate(NavigationDirection::Forward);
        });

        self.imp().category_box.append(&btn);
    }

    fn load_recent_box(&self) {
        let client = CatalogueApplication::client(&CatalogueApplication::default());
        let packages = client.get_recently_updated_packages(Some(12));

        for pkg in packages.iter() {
            let btn = AppTile::new(pkg.clone());
            self.imp().recent_box.append(&btn);
        }
    }
}
