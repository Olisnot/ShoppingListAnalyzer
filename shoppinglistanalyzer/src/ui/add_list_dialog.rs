use gtk4::*;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_add_list_dialog(parent: &ApplicationWindow) {
    let dialog = Dialog::builder()
        .title("Dynamic Form")
        .transient_for(parent)
        .modal(true)
        .default_width(580)
        .default_height(800)
        .build();
    
    let content_area = dialog.content_area();
    content_area.set_margin_end(20);
    content_area.set_spacing(10);

    let main_container = Box::new(Orientation::Vertical, 15);
    
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_propagate_natural_height(true);
    
    let form_box = Box::new(Orientation::Vertical, 15);
    form_box.set_margin_start(10);
    form_box.set_margin_end(10);

    scrolled_window.set_child(Some(&form_box));

    main_container.append(&scrolled_window);

    let name_label = Label::new(Some("Name"));
    name_label.set_margin_start(60);
    let price_label = Label::new(Some("Price"));
    price_label.set_margin_start(110);
    let category_label = Label::new(Some("Category"));
    category_label.set_margin_start(110);

    let label_box = Box::new(Orientation::Horizontal, 15);
    label_box.append(&name_label);
    label_box.append(&price_label);
    label_box.append(&category_label);
    form_box.append(&label_box);
    
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
    dialog.add_button("Next", gtk4::ResponseType::Accept);
    
    dialog.connect_response(|dialog, response| {
        if response == gtk4::ResponseType::Accept {
            println!("Form submitted!");
        }
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
                // Remove this row from the form
                form_box_clone_inner.remove(&item_box_clone_inner);
            }
            dialog.close();
        });
        
        confirm_dialog.present();
    });
    
    let name_entry = Entry::new();
    let price_entry = Entry::new();
    
    name_entry.set_hexpand(true);
    price_entry.set_hexpand(true);
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
