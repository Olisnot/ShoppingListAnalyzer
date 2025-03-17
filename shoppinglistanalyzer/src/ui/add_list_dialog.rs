use gtk4::*;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use sqlite::data_structures::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::sqlite;

pub fn show_add_list_dialog(parent: &ApplicationWindow) {
    let parent_clone = parent.clone();
    let dialog = Dialog::builder()
        .title("Dynamic Form")
        .transient_for(&parent_clone)
        .modal(true)
        .default_width(580)
        .default_height(800)
        .build();
    
    let content_area = dialog.content_area();
    content_area.set_margin_end(20);
    content_area.set_spacing(10);

    let main_container = Box::new(Orientation::Vertical, 15);

    let date_box = Box::new(Orientation::Vertical, 5);
    let date_button = Button::with_label("Select date");

    let popover = Popover::new();
    popover.set_has_arrow(false);
    popover.set_child(Some(&Calendar::new()));

    popover.set_parent(&date_button);
    let popover_clone = popover.clone();
    date_button.connect_clicked(move |_| {
        popover_clone.popup();
    });

    let date_button_clone = date_button.clone();
    let popover_clone2 = popover.clone();
    if let Some(calendar) = popover.child().and_downcast::<Calendar>() {
        calendar.connect_day_selected(move |cal| {
            let date = cal.date();
            date_button_clone.set_label(&format!("{}-{}-{}", date.year(), date.month(), date.day_of_month()));
            popover_clone2.popdown();
        });
    }

    date_box.append(&date_button);
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
    let dialog_clone = dialog.clone();
    
    add_form_row(&form_box_ref.borrow(), &dialog_clone);
    
    let add_button_container = Box::new(Orientation::Horizontal, 0);
    add_button_container.set_hexpand(true);
    add_button_container.set_margin_top(5);
    add_button_container.set_margin_bottom(10);

    let add_button = Button::with_label("+");
    add_button.set_hexpand(true);
    add_button_container.append(&add_button);

    form_box_ref.borrow().append(&add_button_container);
    
    let form_box_ref_clone = Rc::clone(&form_box_ref);
    
    add_button.connect_clicked(move |button| {
        let container = button.parent().unwrap();
        form_box_ref_clone.borrow().remove(&container);

        add_form_row(&form_box_ref_clone.borrow(), &dialog_clone);

        form_box_ref_clone.borrow().append(&container);
    });
    
    main_container.append(&add_button);

    content_area.append(&main_container);
    
    dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
    dialog.add_button("Submit", gtk4::ResponseType::Accept);
    
    let form_box_ref_clone_2 = Rc::clone(&form_box_ref);
    dialog.connect_response(move|dialog, response| {
        if response == gtk4::ResponseType::Accept {
            println!("Form submitted!");
            let date_string: String = date_button.label().unwrap().to_string();
            parse_add_database(&form_box_ref_clone_2.borrow(), date_string);
        }
        parent_clone.queue_draw();
        dialog.close();
    });
    
    dialog.present();
}

fn add_form_row(form_box: &Box, parent_dialog: &Dialog) {
    let item_box = gtk4::Box::new(Orientation::Horizontal, 10);

    let remove_button = Button::with_label("✕");
    remove_button.set_tooltip_text(Some("Remove this item"));

    let item_box_clone = item_box.clone();
    let form_box_clone = form_box.clone();
    
    let parent_dialog_clone = parent_dialog.clone();
    
    remove_button.connect_clicked(move |_| {
        let confirm_dialog = MessageDialog::builder()
            .transient_for(&parent_dialog_clone)
            .modal(true)
            .message_type(MessageType::Question)
            .buttons(ButtonsType::YesNo)
            .text("Delete this item?")
            .build();
        
        confirm_dialog.set_default_response(ResponseType::No);
        
        let item_box_clone_inner = item_box_clone.clone();
        let form_box_clone_inner = form_box_clone.clone();
        
        confirm_dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Yes {
                form_box_clone_inner.remove(&item_box_clone_inner);
            }
            dialog.close();
        });
        
        confirm_dialog.present();
    });
    
    let name_entry = Entry::new();
    let price_entry = Entry::new();
    
    name_entry.set_hexpand(true);
    name_entry.set_placeholder_text(Some("Name"));
    price_entry.set_hexpand(true);
    price_entry.set_placeholder_text(Some("Price"));
    price_entry.set_input_purpose(gtk4::InputPurpose::Number);

    let category_combo = ComboBoxText::new();
    
    category_combo.append(Some("Protein"), "Protein");
    category_combo.append(Some("Fruit/Vegetable"), "Fruit/Vegetable");
    category_combo.append(Some("Dairy"), "Dairy");
    category_combo.append(Some("Carbohydrate"), "Carbohydrate");
    category_combo.append(Some("Fat/Oil"), "Fat/Oil");
    category_combo.append(Some("Unhealthy"), "Unhealthy");
    category_combo.append(Some("Hygiene"), "Hygiene");
    category_combo.append(Some("Miscellaneous"), "Miscellaneous");
    
    category_combo.set_active(Some(0));
    item_box.append(&remove_button);
    
    item_box.append(&name_entry);
    item_box.append(&price_entry);
    item_box.append(&category_combo);
    
    form_box.append(&item_box);
}

fn parse_add_database(form_box: &Box, date: String) {
    println!("start add to db");
    let mut items: Vec<Item> = Vec::new();

    let mut name: String = String::new();
    let mut price: f32 = 0.0;
    let mut category: String = String::new();


    let mut current_child = form_box.first_child();
    while let Some(child) = current_child {
        let root_box = child.downcast_ref::<gtk4::Box>().unwrap();
        let mut inner_current_child = root_box.first_child();
        while let Some(inner_child) = inner_current_child {
            let type_info = inner_child.type_();
            println!("Widget type: {}", type_info.name());
            if inner_child.is::<gtk4::Entry>() {
                let entry = inner_child.downcast_ref::<gtk4::Entry>().unwrap();
                if let Some(placeholder_text) = entry.placeholder_text() {
                    if  placeholder_text == "Name" {
                        println!("name");
                        name = entry.text().to_string();
                    }
                    else if placeholder_text == "Price" {
                        println!("price");
                        price = entry.text().parse().unwrap();
                    }
                }
            }
            else if inner_child.is::<gtk4::ComboBoxText>() {
                let combo = inner_child.downcast_ref::<gtk4::ComboBoxText>().unwrap();
                if let Some(category_text) = combo.active_text() {
                    category = category_text.to_string();
                }
            }
            inner_current_child = inner_child.next_sibling();
        }

        if !name.is_empty() && price > 0.0 {
            let current_item = Item::new(0, name.clone(), category.clone(), price);
            current_item.print_item();
            items.push(current_item);
        }

        current_child = child.next_sibling();
    }

    items.remove(items.len()-1);

    let the_list: List = List::new(0, items, date);
    sqlite::store_list(&the_list);
}
