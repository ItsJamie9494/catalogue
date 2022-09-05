/* widgets/app-tile.rs
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
    glib::{self, Object},
    prelude::*,
    CompositeTemplate,
};

use crate::core::package::Package;

mod imp {
    use std::cell::RefCell;

    use glib::{BindingFlags, ParamSpec, ParamSpecObject, Value};
    use gtk::{Button, Image, Label};
    use once_cell::sync::Lazy;

    use crate::core::package::Package;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/itsjamie/Catalogue/app-tile.ui")]
    pub struct AppTile {
        #[template_child]
        pub icon: TemplateChild<Image>,
        #[template_child]
        pub title: TemplateChild<Label>,

        pub package: RefCell<Package>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppTile {
        const NAME: &'static str = "CatalogueAppTile";
        type Type = super::AppTile;
        type ParentType = Button;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AppTile {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::builder("package", Package::static_type()).build()]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "package" => {
                    let package: Package = value
                        .get()
                        .expect("The value needs to be of type `AsCategory`.");

                    package
                        .bind_property("icon", &self.icon.get(), "gicon")
                        .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
                        .build();

                    package
                        .bind_property("name", &self.title.get(), "label")
                        .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
                        .build();

                    self.package.replace(package);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "package" => self.package.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for AppTile {}
    impl ButtonImpl for AppTile {}
}

glib::wrapper! {
    pub struct AppTile(ObjectSubclass<imp::AppTile>)
        @extends gtk::Widget, gtk::Button;
}

impl AppTile {
    pub fn new(package: Package) -> Self {
        Object::new(&[("package", &package)]).expect("Failed to create AppTile")
    }
}
