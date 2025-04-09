use charming::{ component::Axis, element::{AxisType, AxisLabel}, series::Line, Chart, ImageFormat, ImageRenderer };
use gtk4::*;
use gtk4::prelude::*;
use gdk_pixbuf::{prelude::PixbufLoaderExt, PixbufLoader};
use std::{rc::Rc, cell::RefCell};
use crate::data_structures::ListItem;
use super::ItemsViewer;

impl ItemsViewer {
    pub fn draw_line_chart(&self, item_id: i64) -> Box {
        let b_box = Box::new(Orientation::Vertical, 12);
        b_box.set_hexpand(true);
        b_box.set_vexpand(true);

        let png_data = self.genereat_line_chart(item_id);
        let loader = PixbufLoader::new();
        loader.write(&png_data).unwrap();
        loader.close().unwrap();

        let pixbuf = loader.pixbuf().unwrap();

        let picture = Picture::for_pixbuf(&pixbuf);

        b_box.append(&picture);
        b_box
    }

    fn genereat_line_chart(&self, item_id: i64) -> Vec<u8> {
        println!("{}", item_id);
        let items = Rc::new(RefCell::new(self.database.borrow().get_items_by_item_id(item_id)));
        let chart = Chart::new()
            .x_axis(Axis::new()
                .type_(AxisType::Category)
                .axis_label(AxisLabel::new().font_size(32))
                .data(self.parse_items_for_dates(Rc::clone(&items))))
            .y_axis(Axis::new()
                .axis_label(AxisLabel::new().font_size(32))
                .type_(AxisType::Value))
            .series(Line::new().data(self.parse_data_from_items_for_line(Rc::clone(&items))));
        let mut renderer = ImageRenderer::new(1680, 720);
        renderer.render_format(ImageFormat::Png, &chart).unwrap()
    }

    fn parse_data_from_items_for_line(&self, items: Rc<RefCell<Vec<ListItem>>>) -> Vec<f64> {
        let mut amounts: Vec<f64> = Vec::new();
        for item in items.borrow().iter() {
            amounts.push(item.price);
        }
        amounts
    }

    fn parse_items_for_dates(&self, items: Rc<RefCell<Vec<ListItem>>>) -> Vec<String> {
        let mut dates: Vec<String> = Vec::new();
        for item in items.borrow().iter() {
            dates.push(format!("({}) {}", item.list_id, item.date.clone()));
        }
        dates
    }
}
