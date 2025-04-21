use charming::{ component::{Axis, Legend, grid::Grid}, element::{AxisType, Color, ItemStyle, AxisLabel, Label, Orient, TextStyle}, series::{Bar, Pie}, Chart, ImageFormat, ImageRenderer };
use gtk4::*;
use gtk4::prelude::*;
use gdk_pixbuf::{prelude::PixbufLoaderExt, PixbufLoader};
use std::{rc::Rc, cell::RefCell};
use crate::data_structures::{ List, Categories};

pub fn create_charts(store: Rc<RefCell<TreeStore>>, lists: Rc<RefCell<Vec<List>>>) -> Box {
    let charts_box = Box::new(Orientation::Vertical, 12);
    charts_box.append(&draw_bar_chart(Rc::clone(&lists)));
    charts_box.append(&draw_pie_chart(Rc::clone(&store)));

    charts_box
}

fn draw_bar_chart(lists: Rc<RefCell<Vec<List>>>) -> Box {
    let b_box = Box::new(Orientation::Vertical, 12);
    b_box.set_hexpand(true);
    b_box.set_vexpand(true);

    let png_data = genereat_bar_chart(lists);
    let loader = PixbufLoader::new();
    loader.write(&png_data).unwrap();
    loader.close().unwrap();

    let pixbuf = loader.pixbuf().unwrap();

    let picture = Picture::for_pixbuf(&pixbuf);

    b_box.append(&picture);
    b_box
}

fn draw_pie_chart(store: Rc<RefCell<TreeStore>>) -> Box {
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

fn genereat_bar_chart(lists: Rc<RefCell<Vec<List>>>) -> Vec<u8> {
    let chart = Chart::new()
        .grid(Grid::new().bottom("800"))
        .x_axis(Axis::new()
            .type_(AxisType::Category)
            .axis_label(AxisLabel::new().font_size(32).rotate(-90.0))
            .data(parse_lists_for_dates(Rc::clone(&lists))))
        .y_axis(Axis::new()
            .axis_label(AxisLabel::new().font_size(32))
            .type_(AxisType::Value))
        .series(Bar::new().data(parse_data_from_lists_for_bar(Rc::clone(&lists))));
    let mut renderer = ImageRenderer::new(1680, 1680);
    renderer.render_format(ImageFormat::Png, &chart).unwrap()
}

fn parse_data_from_lists_for_bar(lists: Rc<RefCell<Vec<List>>>) -> Vec<f64> {
    let mut amounts: Vec<f64> = Vec::new();
    for list in lists.borrow().iter() {
        amounts.push(list.get_total_cost());
    }
    amounts
}

fn parse_lists_for_dates(lists: Rc<RefCell<Vec<List>>>) -> Vec<String> {
    let mut dates: Vec<String> = Vec::new();
    for list in lists.borrow().iter() {
        dates.push(format!("({}) {}", list.id, list.date.clone()));
    }
    dates
}

fn generate_pie_chart(store: Rc<RefCell<TreeStore>>) -> Vec<u8> {
    let chart = Chart::new()
        .legend(Legend::new().orient(Orient::Vertical).left("left")
            .text_style(TextStyle::new()
                .font_size(42)
                .color(Color::Value("White".to_string()))
            ))
        .series(Pie::new()
            .item_style(ItemStyle::new().border_radius(8))
            .label(Label::new().show(false))
            .data(parse_data_from_store_for_pie(store)));
    let mut renderer = ImageRenderer::new(1680, 720);
    renderer.render_format(ImageFormat::Png, &chart).unwrap()
}

fn parse_data_from_store_for_pie(store: Rc<RefCell<TreeStore>>) -> Vec<(f64, String)> {
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
        (proteins, format!("{} - {:.2}", Categories::Protein.to_cat_string(), proteins/2.0)),
        (fruit_vegtabable, format!("{} - {:.2}" ,Categories::FruitsVegetables.to_cat_string(), fruit_vegtabable/2.0)),
        (dairy, format!("{} - {:.2}", Categories::Dairy.to_cat_string(), dairy/2.0)),
        (fat_oil, format!("{} - {:.2}", Categories::FatsOils.to_cat_string(), fat_oil/2.0)),
        (carbohydrate, format!("{} - {:.2}", Categories::Carbohydrates.to_cat_string(), carbohydrate/2.0)),
        (unhealthy, format!("{} - {:.2}", Categories::Unhealthy.to_cat_string(), unhealthy/2.0)),
        (hygiene, format!("{} - {:.2}", Categories::Hygiene.to_cat_string(), hygiene/2.0)),
        (misc, format!("{} - {:.2}", Categories::Misc.to_cat_string(), misc/2.0)),
    };
    data
}
