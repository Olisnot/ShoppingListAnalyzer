use charming::{ component::Legend, element::{Color, ItemStyle, Label, Orient, TextStyle}, series::Pie, Chart, ImageFormat, ImageRenderer };
use gtk4::*;
use gtk4::prelude::*;
use gdk_pixbuf::{prelude::PixbufLoaderExt, PixbufLoader};
use std::{rc::Rc, cell::RefCell};
use crate::data_structures::Categories;

pub fn draw(store: Rc<RefCell<ListStore>>) -> Box {
    let p_box = Box::new(Orientation::Vertical, 12);
    p_box.set_hexpand(true);
    p_box.set_vexpand(true);

    let png_data = generate_pie_chart(store);
    let loader = PixbufLoader::new();
    loader.write(&png_data).unwrap();
    loader.close().unwrap();

    let pixbuf = loader.pixbuf().unwrap();

    let picture = Picture::for_pixbuf(&pixbuf);

    p_box.append(&picture);
    p_box
}

fn generate_pie_chart(store: Rc<RefCell<ListStore>>) -> Vec<u8> {
    let chart = Chart::new()
        .legend(Legend::new().orient(Orient::Vertical).left("left")
            .text_style(TextStyle::new()
                .font_size(20)
                .color(Color::Value("White".to_string()))
            ))
        .series(Pie::new()
            .item_style(ItemStyle::new().border_radius(8))
            .label(Label::new().show(false))
            .data(parse_data_from_store(store)));
    let mut renderer = ImageRenderer::new(1120, 480);
    println!("saved chart");
    renderer.render_format(ImageFormat::Png, &chart).unwrap()
}

fn parse_data_from_store(store: Rc<RefCell<ListStore>>) -> Vec<(f64, String)> {
    let mut proteins = 0.0;
    let mut fruit_vegtabable = 0.0;
    let mut dairy = 0.0;
    let mut fat_oil = 0.0;
    let mut carbohydrate = 0.0;
    let mut unhealthy = 0.0;
    let mut hygiene = 0.0;
    let mut misc = 0.0;
    store.borrow().foreach(|_model, _path, iter| {
        let category = store.borrow().get::<String>(iter, 1);
        let price = store.borrow().get::<f64>(iter, 2); 
        if category == Categories::Protein.to_cat_string() {
            proteins += price;
        } else if category == Categories::FruitsVegetables.to_cat_string() {
            fruit_vegtabable += price;
        } else if category == Categories::Dairy.to_cat_string() {
            dairy += price;
        } else if category == Categories::FatsOils.to_cat_string() {
            fat_oil += price;
        } else if category == Categories::Carbohydrates.to_cat_string() {
            carbohydrate += price;
        } else if category == Categories::Unhealthy.to_cat_string() {
            unhealthy += price;
        } else if category == Categories::Hygiene.to_cat_string() {
            hygiene += price;
        } else if category == Categories::Misc.to_cat_string() || !category.is_empty() {
            misc += price;
        } 
        false 
    });
    let data: Vec<(f64, String)> = vec!{
        (proteins, Categories::Protein.to_cat_string()),
        (fruit_vegtabable, Categories::FruitsVegetables.to_cat_string()),
        (dairy, Categories::Dairy.to_cat_string()),
        (fat_oil, Categories::FatsOils.to_cat_string()),
        (carbohydrate, Categories::Carbohydrates.to_cat_string()),
        (unhealthy, Categories::Unhealthy.to_cat_string()),
        (hygiene, Categories::Hygiene.to_cat_string()),
        (misc, Categories::Misc.to_cat_string()),
    };
    data
}
