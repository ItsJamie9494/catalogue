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
use appstream::Category;
use gtk::{
    glib::{self, Object},
    prelude::*,
    CompositeTemplate,
};

mod imp {
    use std::cell::RefCell;

    use glib::{ParamSpec, ParamSpecObject, Value};
    use gtk::FlowBox;
    use once_cell::sync::Lazy;

    use crate::widgets::page::{Page, PageImpl};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/itsjamie/Catalogue/category-page.ui")]
    pub struct CategoryPage {
        #[template_child]
        pub editor_box: TemplateChild<FlowBox>,
        #[template_child]
        pub recent_box: TemplateChild<FlowBox>,
        #[template_child]
        pub more_box: TemplateChild<FlowBox>,

        pub category: RefCell<Category>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CategoryPage {
        const NAME: &'static str = "CatalogueCategoryPage";
        type Type = super::CategoryPage;
        type ParentType = Page;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CategoryPage {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::builder("category", Category::static_type()).build()]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "category" => {
                    let category: Category = value
                        .get()
                        .expect("The value needs to be of type `AsCategory`.");

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
    impl WidgetImpl for CategoryPage {}
    impl PageImpl for CategoryPage {}
}

glib::wrapper! {
    pub struct CategoryPage(ObjectSubclass<imp::CategoryPage>)
        @extends gtk::Widget, gtk::Button;
}

impl CategoryPage {
    pub fn new(category: Category) -> Self {
        Object::new(&[("category", &category)]).expect("Failed to create CategoryPage")
    }
}
