use gtk4::{* ,prelude::*};
use crate::data_structures::ListItem;

use super::super::ItemsViewer;

impl ItemsViewer {
    pub fn create_edit_item_dialog(&self, parent: &ApplicationWindow, item_id: i64) {
        let parent_clone = parent.clone();
        let dialog = Dialog::builder()
            .title("Edit Item")
            .transient_for(&parent_clone)
            .modal(true)
            .default_width(300)
            .default_height(500)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Submit", ResponseType::Accept);
        let content_area = dialog.content_area();
        content_area.set_margin_end(20);
        content_area.set_spacing(10);

        let item_variants = self.database.borrow().get_items_by_item_id(self.items[item_id as usize].id);

        let name_box = Box::new(Orientation::Horizontal, 10);
        let name_label = Label::new(Some("Name\t"));
        name_box.append(&name_label);
        let name_entry = Entry::new();
        name_entry.set_text(&item_variants[0].name);
        name_box.append(&name_entry);
        content_area.append(&name_box);

        let category_box = Box::new(Orientation::Horizontal, 10);
        let category_label = Label::new(Some("Category\t"));
        category_box.append(&category_label);
        let cat_combo = create_combo_box(item_variants[0].category.clone());
        category_box.append(&cat_combo);
        content_area.append(&category_box);

        content_area.append(&create_prices_scrolled_window(item_variants));

        dialog.connect_response(move|dialog, response| {
            if response == ResponseType::Accept {
                println!("Form submitted!");
            }
            dialog.close();
        });
        dialog.present();
    }
}

fn create_combo_box(category: String) -> ComboBoxText {
    let category_combo = ComboBoxText::new();
    
    category_combo.append(Some("Protein"), "Protein");
    category_combo.append(Some("Fruit/Vegetable"), "Fruit/Vegetable");
    category_combo.append(Some("Dairy"), "Dairy");
    category_combo.append(Some("Carbohydrate"), "Carbohydrate");
    category_combo.append(Some("Fat/Oil"), "Fat/Oil");
    category_combo.append(Some("Unhealthy"), "Unhealthy");
    category_combo.append(Some("Hygiene"), "Hygiene");
    category_combo.append(Some("Miscellaneous"), "Miscellaneous");

    category_combo.set_active_id(Some(&category));
    category_combo
}

fn create_prices_scrolled_window(items: Vec<ListItem>) -> ScrolledWindow {
    let window = ScrolledWindow::new();
    window.set_vexpand(true);
    let main_box = Box::new(Orientation::Vertical, 10);
    for item in items.iter() {
        let item_box = Box::new(Orientation::Horizontal, 10);
        let list_label = Label::new(Some(&format!("({}) {}", item.list_id, item.date)));

        let price_entry = Entry::new();
        price_entry.set_text(&item.price.to_string());
        price_entry.connect_changed(|entry| {
            let text = entry.text();
            let mut filtered = String::new();
            let mut dot_seen = false;

            for char in text.chars() {
                if char.is_ascii_digit() {
                    filtered.push(char);
                } else if char == '.' && !dot_seen {
                    filtered.push(char);
                    dot_seen = true;
                }
            }

            if text != filtered {
                let pos = entry.position();
                entry.set_text(&filtered);
                entry.set_position(pos.saturating_sub(1));
            }
        });

        item_box.append(&list_label);
        item_box.append(&price_entry);
        main_box.append(&item_box);
    }
    window.set_child(Some(&main_box));
    window
}
