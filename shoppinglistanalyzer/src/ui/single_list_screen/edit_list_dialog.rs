use std::{rc::Rc, cell::RefCell};
use gtk4::*;
use gtk4::prelude::*;
use gtk4::glib::Type;
use super::SingleList;
use crate::ui::add_list_dialog::{RowTuple, build_form_row, refresh_stack};
use crate::categorization::categorize;
use crate::data_structures::{List, Item};


impl SingleList {
    pub fn show_edit_list_dialog(&self, parent: &ApplicationWindow) {
        let list = self.database.borrow().get_list(self.active_list_id);

        let parent_clone = parent.clone();
        let dialog = Dialog::builder()
            .title("Add List")
            .transient_for(&parent_clone)
            .modal(true)
            .default_width(580)
            .default_height(800)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Submit", ResponseType::Accept);

        let content_area = dialog.content_area();
        content_area.set_margin_end(20);
        content_area.set_spacing(10);

        let main_container = Box::new(Orientation::Vertical, 15);

        let date_box = Box::new(Orientation::Vertical, 5);
        let date_button = Rc::new(RefCell::new(Button::with_label("Select date")));

        let calendar = Calendar::new();
        self.set_date(&calendar, list.date.clone());
        date_button.borrow().set_label(&list.date);

        let popover = Popover::new();
        popover.set_has_arrow(false);
        popover.set_child(Some(&calendar));

        popover.set_parent(&*date_button.borrow());
        let popover_clone = popover.clone();
        date_button.borrow().connect_clicked(move |_| {
            popover_clone.popup();
        });

        let date_button_clone = date_button.clone();
        let popover_clone2 = popover.clone();
        if let Some(calendar) = popover.child().and_downcast::<Calendar>() {
            calendar.connect_day_selected(move |cal| {
                let date = cal.date();
                date_button_clone.borrow().set_label(&format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day_of_month()));
                popover_clone2.popdown();
            });
        }

        date_box.append(&*date_button.borrow());
        main_container.append(&date_box);

        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_policy(PolicyType::Never, PolicyType::Automatic);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_propagate_natural_height(true);

        let form_box = Box::new(Orientation::Vertical, 15);
        form_box.set_margin_start(10);
        form_box.set_margin_end(10);

        scrolled_window.set_child(Some(&form_box));

        main_container.append(&scrolled_window);

        let margin_box = Box::new(Orientation::Horizontal, 15);
        form_box.append(&margin_box);

        let form_box_ref = Rc::new(RefCell::new(form_box));

        let store = Rc::new(RefCell::new(ListStore::new(&[Type::STRING])));

        let rows: RowTuple = Rc::new(RefCell::new(Vec::new()));

        let db_items = self.database.borrow().get_items(); 
        for item in db_items.iter() {
            store.borrow().set(&store.borrow().append(), &[(0, &item.name)]);
        }

        for item in list.items.iter() {
            let row = self.build_form_row_with_item(Rc::clone(&store), &dialog, item, &Rc::clone(&rows));
            form_box_ref.borrow().append(&row);
        }

        let add_button_container = Rc::new(Box::new(Orientation::Horizontal, 0));
        add_button_container.set_hexpand(true);
        add_button_container.set_margin_top(5);
        add_button_container.set_margin_bottom(10);

        let add_button = Button::with_label("+");
        add_button.set_hexpand(true);
        add_button_container.append(&add_button);

        form_box_ref.borrow().append(&*add_button_container);

        let form_box_clone = Rc::clone(&form_box_ref);
        let store_clone = Rc::clone(&store);
        let db_clone = Rc::clone(&self.database);
        let rows_clone = Rc::clone(&rows);
        let dialog_clone2 = dialog.clone();
        let add_button_container = Rc::clone(&add_button_container);

        add_button.connect_clicked(move |_| {
            let new_row = build_form_row(Rc::clone(&store_clone), &dialog_clone2, Rc::clone(&rows_clone), Rc::clone(&db_clone));

            let mut prev = None;
            let mut child = form_box_clone.borrow().first_child();

            while let Some(ref c) = child {
                if c == &*add_button_container {
                    break;
                }
                prev = child.clone();
                child = c.next_sibling();
            }

            if let Some(prev) = prev {
                form_box_clone.borrow().insert_child_after(&new_row, Some(&prev));
            } else {
                form_box_clone.borrow().prepend(&new_row);
            }
        });

        main_container.append(&add_button);

        content_area.append(&main_container);

        let buttons_grid = Grid::new();

        let delete_list_button = Button::with_label("Delete List");
        let db_clone = Rc::clone(&self.database);
        let self_rc_1 = self.self_ref.as_ref().unwrap().clone();
        let dialog_clone = dialog.clone();
        delete_list_button.connect_clicked(move |_| {
            let parent_dialog_clone = dialog_clone.clone();

            let confirm_dialog = MessageDialog::builder()
                .transient_for(&parent_dialog_clone)
                .modal(true)
                .message_type(MessageType::Question)
                .buttons(ButtonsType::YesNo)
                .text("The list will be PERMANENTLY deleted\nContinue?")
                .build();

            confirm_dialog.set_default_response(ResponseType::No);

            let db_clone_2 = Rc::clone(&db_clone);
            let dialog_clone_2 = dialog_clone.clone();
            let self_rc_2 = Rc::clone(&self_rc_1);
            confirm_dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Yes {
                    db_clone_2.borrow().delete_list(self_rc_2.borrow().active_list_id);
                    let stack = find_parent_stack(&Widget::from(self_rc_2.borrow().list_selector.clone())).unwrap();
                    refresh_stack(&stack, Rc::clone(&db_clone_2));
                    dialog_clone_2.close();
                }
                dialog.close();
            });
            confirm_dialog.present();
        });
        buttons_grid.attach(&delete_list_button, 2, 0, 1, 1);

        let categorization_button = Button::with_label("Categorize");
        categorization_button.set_hexpand(true);
        let rows_clone_3 = Rc::clone(&rows);
        let db_clone_2 = Rc::clone(&self.database);
        categorization_button.connect_clicked(move |_| {
            categorize(Rc::clone(&rows_clone_3), Rc::clone(&db_clone_2));
        });
        buttons_grid.attach(&categorization_button, 0, 0, 2, 1);

        content_area.append(&buttons_grid);

        let rows_clone_2 = Rc::clone(&rows);
        let self_rc = self.self_ref.as_ref().unwrap().clone();
        dialog.connect_response(move|dialog, response| {
            if response == ResponseType::Accept {
                let date_string: String = date_button.borrow().label().unwrap().to_string();
                self_rc.borrow_mut().apply_edited_list(date_string, Rc::clone(&rows_clone_2));
            }
            parent_clone.queue_draw();
            dialog.close();
        });

        dialog.present();
    }

    fn set_date(&self, calendar:&Calendar, date:String) {
        let split_date = date.split('-').collect::<Vec<&str>>();
        let year = split_date[0];
        let month = split_date[1];
        let day = split_date[2];

        calendar.set_day(day.parse().unwrap());
        calendar.set_month(month.parse::<i32>().unwrap() - 1);
        calendar.set_year(year.parse().unwrap());
    }

    fn apply_edited_list(&mut self, date: String, rows: RowTuple) {
        let stack = find_parent_stack(&Widget::from(self.list_selector.clone())).unwrap();
        let mut items: Vec<Item> = Vec::new();

        for (name, price, category) in rows.borrow_mut().iter() {
            items.push(Item::new(0, name.text().to_string(), category.active_text().unwrap().to_string(), price.text().to_string().parse().unwrap()));
        }

        let the_list: List = List::new(self.active_list_id, items, date.clone());
        self.database.borrow().update_list(&the_list);
        refresh_stack(&stack, self.database.clone());
    }

    fn build_form_row_with_item(&self, store: Rc<RefCell<ListStore>>, parent_dialog: &Dialog, item: &Item, rows: &RowTuple) -> gtk4::Box {
        let item_box = Box::new(Orientation::Horizontal, 10);

        let remove_button = Button::with_label("✕");
        remove_button.set_tooltip_text(Some("Remove this item"));

        let name_entry = Entry::new();
        let price_entry = Entry::new();
        let category_combo = ComboBoxText::new();

        name_entry.set_hexpand(true);
        name_entry.set_placeholder_text(Some("Name"));
        name_entry.set_text(&item.name);

        price_entry.set_hexpand(true);
        price_entry.set_placeholder_text(Some("Price"));
        price_entry.set_text(&format!("{}", item.price));
        price_entry.set_input_purpose(InputPurpose::Number);

        let completion = EntryCompletion::new();
        completion.set_model(Some(&*store.borrow()));
        completion.set_text_column(0);
        completion.set_inline_completion(true);
        completion.set_popup_completion(true);
        name_entry.set_completion(Some(&completion));

        category_combo.append(Some("Protein"), "Protein");
        category_combo.append(Some("Fruit/Vegetable"), "Fruit/Vegetable");
        category_combo.append(Some("Dairy"), "Dairy");
        category_combo.append(Some("Carbohydrate"), "Carbohydrate");
        category_combo.append(Some("Fat/Oil"), "Fat/Oil");
        category_combo.append(Some("Unhealthy"), "Unhealthy");
        category_combo.append(Some("Hygiene"), "Hygiene");
        category_combo.append(Some("Miscellaneous"), "Miscellaneous");
        category_combo.set_active_id(Some(&item.category));
        category_combo.set_sensitive(false);

        let combo_clone = category_combo.clone();
        let existing_items = self.database.borrow().get_items();
        name_entry.connect_changed(move |text| {
            for i in existing_items.iter() {
                if text.text() == *i.name {
                    combo_clone.set_sensitive(false);
                    combo_clone.set_active_id(Some(&i.category));
                    break;
                }
                else {
                    combo_clone.set_sensitive(true);
                }
            }
        });

        let item_box_clone = item_box.clone();
        let parent_dialog_clone = parent_dialog.clone();
        let rows_clone = Rc::clone(rows);

        let row = (name_entry.clone(), price_entry.clone(), category_combo.clone());
        let row_clone = row.clone();
        remove_button.connect_clicked(move |_| {
            let confirm_dialog = MessageDialog::builder()
                .transient_for(&parent_dialog_clone)
                .modal(true)
                .message_type(MessageType::Question)
                .buttons(ButtonsType::YesNo)
                .text("Delete this item?")
                .build();

            confirm_dialog.set_default_response(ResponseType::No);
            let rows_clone_2 = Rc::clone(&rows_clone);
            let row_clone_2 = row_clone.clone();

            let item_box_inner = item_box_clone.clone();
            confirm_dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Yes {
                    if let Some(parent) = item_box_inner.parent() {
                        let index = rows_clone_2.borrow_mut().iter().position(|x| *x == row_clone_2).unwrap();
                        rows_clone_2.borrow_mut().remove(index);
                        parent.downcast::<gtk4::Box>().unwrap().remove(&item_box_inner);
                    }
                }
                dialog.close();
            });

            confirm_dialog.present();
        });

        rows.borrow_mut().push(row);

        item_box.append(&remove_button);
        item_box.append(&name_entry);
        item_box.append(&price_entry);
        item_box.append(&category_combo);

        item_box
    }
}

fn find_parent_stack(widget: &Widget) -> Option<Stack> {
    let mut current = widget.parent();

    while let Some(parent) = current {
        if let Ok(stack) = parent.clone().downcast::<Stack>() {
            return Some(stack);
        }
        current = parent.parent();
    }

    None
}
