/* widgets/category-tile.rs
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
use appstream::prelude::*;
use appstream::Category;
use gtk::{
    glib::{self, Object},
    prelude::*,
    CompositeTemplate,
};

mod imp {
    use std::cell::RefCell;

    use glib::{BindingFlags, ParamSpec, ParamSpecObject, Value};
    use gtk::{Button, Image, Label};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/itsjamie/Catalogue/category-tile.ui")]
    pub struct CategoryTile {
        #[template_child]
        pub category_icon: TemplateChild<Image>,
        #[template_child]
        pub category_label: TemplateChild<Label>,

        pub category: RefCell<Category>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CategoryTile {
        const NAME: &'static str = "CatalogueCategoryTile";
        type Type = super::CategoryTile;
        type ParentType = Button;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CategoryTile {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::builder("category", Category::static_type()).build()]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "category" => {
                    let category: Category = value
                        .get()
                        .expect("The value needs to be of type `AsCategory`.");

                    category
                        .bind_property("icon", &self.category_icon.get(), "icon-name")
                        .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
                        .build();

                    category
                        .bind_property("name", &self.category_label.get(), "label")
                        .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
                        .build();

                    obj.add_css_class(&format!("tile-{}", category.name().unwrap().to_lowercase()));

                    self.category.replace(category);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "category" => self.category.borrow_mut().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for CategoryTile {}
    impl ButtonImpl for CategoryTile {}
}

glib::wrapper! {
    pub struct CategoryTile(ObjectSubclass<imp::CategoryTile>)
        @extends gtk::Widget, gtk::Button;
}

impl CategoryTile {
    pub fn new(category: Category) -> Self {
        Object::new(&[("category", &category)]).expect("Failed to create CategoryTile")
    }

    pub fn category(&self) -> Category {
        self.imp().category.borrow().clone()
    }
}
