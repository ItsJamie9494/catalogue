/* widgets/installed_row.rs
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
use adw::{prelude::*, Bin};
use gtk::glib::{self, Object};

use crate::core::package::Package;

mod imp {
    use adw::ActionRow;
    use glib::{BindingFlags, ParamSpec, ParamSpecObject, Value};
    use gtk::Image;
    use once_cell::sync::Lazy;
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default)]
    pub struct InstalledRow {
        row: RefCell<ActionRow>,
        image: RefCell<Image>,

        pub package: RefCell<Package>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InstalledRow {
        const NAME: &'static str = "CatalogueInstalledRow";
        type Type = super::InstalledRow;
        type ParentType = Bin;
    }

    impl ObjectImpl for InstalledRow {
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

                    let row = self.row.borrow().clone();
                    let image = self.image.borrow().clone();

                    package
                        .bind_property("name", &row, "title")
                        .flags(BindingFlags::SYNC_CREATE)
                        .build();

                    package
                        .bind_property("icon", &image, "gicon")
                        .flags(BindingFlags::SYNC_CREATE)
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

        fn constructed(&self, obj: &Self::Type) {
            let row = self.row.borrow().clone();
            let image = self.image.borrow().clone();
            obj.set_child(Some(&row));

            image.set_pixel_size(64);
            image.add_css_class("icon-dropshadow");
            row.add_prefix(&image);

            self.parent_constructed(obj);
        }
    }
    impl WidgetImpl for InstalledRow {}
    impl BinImpl for InstalledRow {}
}

glib::wrapper! {
    pub struct InstalledRow(ObjectSubclass<imp::InstalledRow>)
    @extends gtk::Widget, Bin,
    @implements gtk::Accessible, gtk::ConstraintTarget, gtk::Actionable;
}

impl InstalledRow {
    pub fn new(package: &Package) -> Self {
        Object::new(&[("package", &package)]).expect("Failed to create InstalledRow")
    }

    pub fn package(&self) -> Package {
        self.imp().package.borrow().clone()
    }
}
