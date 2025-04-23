use std::{rc::Rc ,cell::RefCell};
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

        let item_variants = Rc::new(RefCell::new(self.database.borrow().get_items_by_item_id(self.items[item_id as usize].id)));

        let name_box = Box::new(Orientation::Horizontal, 10);
        let name_label = Label::new(Some("Name\t"));
        name_box.append(&name_label);
        let name_entry = Rc::new(RefCell::new(Entry::new()));
        name_entry.borrow().set_text(&item_variants.borrow()[0].name);
        name_box.append(&*name_entry.borrow());
        content_area.append(&name_box);

        let category_box = Box::new(Orientation::Horizontal, 10);
        let category_label = Label::new(Some("Category\t"));
        category_box.append(&category_label);
        let cat_combo = create_combo_box(item_variants.borrow()[0].category.clone());
        category_box.append(&*cat_combo.borrow());
        content_area.append(&category_box);

        let prices_entries: Rc<RefCell<Vec<Entry>>> = Rc::new(RefCell::new(Vec::new()));
        let prices_window = create_prices_scrolled_window(Rc::clone(&item_variants), Rc::clone(&prices_entries));
        content_area.append(&*prices_window.borrow());

        let self_rc = self.self_ref.as_ref().unwrap().clone();
        dialog.connect_response(move|dialog, response| {
            if response == ResponseType::Accept {
                self_rc.borrow().save_edited_item(Rc::clone(&item_variants), Rc::clone(&name_entry), Rc::clone(&cat_combo), Rc::clone(&prices_entries));
                self_rc.borrow_mut().refresh_items();
                self_rc.borrow_mut().create_list_view();
            }
            dialog.close();
        });
        dialog.present();
    }

    fn save_edited_item(&self, variants: Rc<RefCell<Vec<ListItem>>>, name: Rc<RefCell<Entry>>, category: Rc<RefCell<ComboBoxText>>, prices: Rc<RefCell<Vec<Entry>>>) {
        let new_name = name.borrow().text();
        let new_category = category.borrow().active_text().unwrap();
        let item_id = variants.borrow()[0].item_id;

        let mut query = format!("
            UPDATE items
            SET Name = \"{}\",
            Category = \"{}\"
            WHERE ItemId = {};

            ", new_name, new_category, item_id);

            for (i, variant) in variants.borrow_mut().iter().enumerate() {
                let temp_query = format!("\n
                    UPDATE listItems
                    SET Price = {}
                    WHERE ListId = {} AND ItemId = {};
                    \n", prices.borrow()[i].text(), variant.list_id, variant.item_id);
                    query.push_str(&temp_query);
            }

        println!("Query: \n {}", query);
            self.database.borrow().connection.as_ref().unwrap().execute(query).unwrap();
    }
}

fn create_combo_box(category: String) -> Rc<RefCell<ComboBoxText>> {
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
    Rc::new(RefCell::new(category_combo))
}

fn create_prices_scrolled_window(items: Rc<RefCell<Vec<ListItem>>>, entries: Rc<RefCell<Vec<Entry>>>) -> Rc<RefCell<ScrolledWindow>> {
    let window = ScrolledWindow::new();
    window.set_vexpand(true);
    let main_box = Box::new(Orientation::Vertical, 10);
    for item in items.borrow().iter() {
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
        entries.borrow_mut().push(price_entry);
        main_box.append(&item_box);
    }
    window.set_child(Some(&main_box));
    Rc::new(RefCell::new(window))
}
