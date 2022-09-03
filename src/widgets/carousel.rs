/* widgets/carousel.rs
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
    glib::{self, clone, Object},
    prelude::*,
    CompositeTemplate,
};

mod imp {
    use super::*;
    use gtk::Button;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/itsjamie/Catalogue/carousel.ui")]
    pub struct Carousel {
        #[template_child]
        pub carousel: TemplateChild<adw::Carousel>,
        #[template_child]
        pub previous_button: TemplateChild<Button>,
        #[template_child]
        pub next_button: TemplateChild<Button>,
    }

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/itsjamie/Catalogue/carousel-tile.ui")]
    pub struct CarouselTile {}

    impl Carousel {
        pub fn move_relative_page(&self, delta: i32) {
            let new_page =
                (self.carousel.position() as i32 + delta + self.carousel.n_pages() as i32)
                    % self.carousel.n_pages() as i32;

            let page = self.carousel.nth_page(new_page.try_into().unwrap());

            self.carousel.scroll_to(&page, true);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Carousel {
        const NAME: &'static str = "CatalogueCarousel";
        type Type = super::Carousel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CarouselTile {
        const NAME: &'static str = "CatalogueCarouselTile";
        type Type = super::CarouselTile;
        type ParentType = gtk::Button;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Carousel {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_callbacks();

            obj.carousel().append(&super::CarouselTile::new());
            obj.carousel().append(&super::CarouselTile::new());
        }
    }
    impl ObjectImpl for CarouselTile {}
    impl WidgetImpl for Carousel {}
    impl WidgetImpl for CarouselTile {}
    impl BoxImpl for Carousel {}
    impl ButtonImpl for CarouselTile {}
}

glib::wrapper! {
    pub struct Carousel(ObjectSubclass<imp::Carousel>)
        @extends gtk::Widget, gtk::Box;
}

glib::wrapper! {
    pub struct CarouselTile(ObjectSubclass<imp::CarouselTile>)
        @extends gtk::Widget, gtk::Button;
}

impl Carousel {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create Carousel")
    }

    pub fn setup_callbacks(&self) {
        self.imp()
            .next_button
            .connect_clicked(clone!(@strong self as this => move |_| {
                this.imp().move_relative_page(1);
            }));
        self.imp()
            .previous_button
            .connect_clicked(clone!(@strong self as this => move |_| {
                this.imp().move_relative_page(-1);
            }));
    }

    pub fn carousel(&self) -> adw::Carousel {
        self.imp().carousel.clone()
    }
}

impl CarouselTile {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create CarouselTile")
    }
}
