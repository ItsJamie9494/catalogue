/* widgets/page.rs
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
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::{self, Object};

mod imp {
    use adw::Clamp;
    use gtk::{
        Align, BinLayout, Box, Buildable, Builder, Orientation, PolicyType, ScrolledWindow, Widget,
    };
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Page {
        content: RefCell<Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Page {
        const NAME: &'static str = "CataloguePage";
        type Type = super::Page;
        type ParentType = Widget;
        type Interfaces = (Buildable,);

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<BinLayout>();
        }
    }

    impl ObjectImpl for Page {
        fn constructed(&self, obj: &Self::Type) {
            let scrolled = ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never)
                .min_content_width(900)
                .build();
            scrolled.set_parent(obj);
            let clamp = Clamp::builder()
                .maximum_size(1000)
                .tightening_threshold(600)
                .build();
            scrolled.set_child(Some(&clamp));
            let content = Box::builder()
                .valign(Align::Start)
                .halign(Align::Center)
                .hexpand(false)
                .orientation(Orientation::Vertical)
                .margin_top(24)
                .margin_bottom(36)
                .margin_start(12)
                .margin_end(12)
                .spacing(12)
                .build();
            clamp.set_child(Some(&content));
            self.content.replace(content);

            self.parent_constructed(obj);
        }

        fn dispose(&self, buildable: &Self::Type) {
            while let Some(child) = buildable.first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Page {}
    impl BuildableImpl for Page {
        fn add_child(
            &self,
            buildable: &Self::Type,
            builder: &Builder,
            child: &Object,
            type_: Option<&str>,
        ) {
            if buildable.first_child().is_none() {
                self.parent_add_child(buildable, builder, child, type_);
            } else {
                self.content
                    .borrow_mut()
                    .append(child.downcast_ref::<Widget>().expect("Expected a Widget"));
            }
        }
    }
}

glib::wrapper! {
    pub struct Page(ObjectSubclass<imp::Page>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Page {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create Page")
    }
}

pub trait PageImpl: WidgetImpl {}

unsafe impl<T: PageImpl> IsSubclassable<T> for Page {}
