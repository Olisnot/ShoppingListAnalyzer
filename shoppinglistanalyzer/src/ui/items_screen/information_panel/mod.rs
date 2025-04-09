mod dialogs;
mod chart;

use gtk4::{*, prelude::*};
use super::ItemsViewer;

impl ItemsViewer {
    pub fn create_information_panel(&mut self, item_id: i64) {
        let info_box = Box::new(Orientation::Vertical, 10);
        let label = Label::new(Some(&self.items[item_id as usize].name));
        label.set_widget_name("info-panel-label");


        let header = HeaderBar::builder()
            .title_widget(&label)
            .show_title_buttons(false)
            .build();
        let edit_item_button = Button::with_label("Edit");
        let get_nutrition_button = Button::with_label("Get Nutrition");
        let button_box = Box::new(Orientation::Horizontal, 10);
        button_box.append(&edit_item_button);
        button_box.append(&get_nutrition_button);
        header.pack_start(&button_box);

        let self_rc = self.self_ref.as_ref().unwrap().clone();
        edit_item_button.connect_clicked(move |button| {
            if let Some(window) = button.root().and_then(|w| w.downcast::<ApplicationWindow>().ok()) {
                self_rc.borrow().create_edit_item_dialog(&window, item_id);
            }

        });

        info_box.append(&header);
        let content_box = Box::new(Orientation::Horizontal, 10);
        content_box.append(&self.create_price_labels(item_id));
        content_box.append(&self.draw_line_chart(self.items[item_id as usize].id));

        info_box.append(&content_box);

        self.information_panel = Some(info_box);
        self.main_content.as_ref().unwrap().set_end_child(self.information_panel.as_ref());
    }

    fn create_price_labels(&self, item_id: i64) -> Box {
        let items = self.database.borrow().get_items_by_item_id(self.items[item_id as usize].id);
        let label_box = Box::new(Orientation::Vertical, 10);
        let max_label = Label::new(None);
        let min_label = Label::new(None);
        let average_label = Label::new(None);
        let price_history_label = Label::new(Some("\n\nPrice History"));

        let mut max = 0.0;
        let mut min = items[0].price;
        let mut average = 0.0;
        for item in items.iter() {
            if item.price > max {
                max = item.price;
            }
            if item.price < min {
                min = item.price;
            }
            average += item.price;
        }
        average /= items.iter().len() as f64;

        max_label.set_text(&format!("Maximum Price: {:.2}", max));
        min_label.set_text(&format!("Minimum Price: {:.2}", min));
        average_label.set_text(&format!("Average Price: {:.2}", average));
        max_label.set_widget_name("price-label");
        min_label.set_widget_name("price-label");
        average_label.set_widget_name("price-label");
        price_history_label.set_widget_name("price-label");

        label_box.append(&max_label);
        label_box.append(&min_label);
        label_box.append(&average_label);
        label_box.append(&price_history_label);
        label_box.append(&self.create_price_history_table(item_id));

        label_box
    }

    fn create_price_history_table(&self, item_id: i64) -> ScrolledWindow {
        let items = self.database.borrow().get_items_by_item_id(self.items[item_id as usize].id);
        let grid = Grid::builder()
            .column_spacing(12)
            .row_spacing(6)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let list_label = Label::new(Some("List"));
        list_label.set_widget_name("bold-label");
        let price_label = Label::new(Some("Price"));
        price_label.set_widget_name("bold-label");
        grid.attach(&list_label, 0, 0, 1, 1);
        grid.attach(&price_label, 1, 0, 1, 1);

        for (i, item) in items.iter().enumerate() {
            grid.attach(&Label::new(Some(&format!("({}) {}", item.list_id, item.date))), 0, (i + 1) as i32, 1, 1);
            grid.attach(&Label::new(Some(&item.price.to_string())), 1, (i + 1) as i32, 1, 1);
        }

        ScrolledWindow::builder()
            .child(&grid)
            .min_content_height(200)
            .vexpand(true)
            .build()
    }
}
