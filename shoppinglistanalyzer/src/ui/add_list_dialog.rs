use gtk4::*;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_add_list_dialog(parent: &ApplicationWindow) {
    // Create a new dialog
    let dialog = Dialog::builder()
        .title("Dynamic Form")
        .transient_for(parent)
        .modal(true)
        .default_width(500)
        .default_height(800)
        .build();
    
    let content_area = dialog.content_area();
    content_area.set_margin_end(20);
    content_area.set_spacing(10);
    
    let form_box = Box::new(Orientation::Vertical, 15);
    content_area.append(&form_box);

    let name_label = Label::new(Some("Name"));
    let price_label = Label::new(Some("Price"));
    price_label.set_margin_start(190);

    let label_box = Box::new(Orientation::Horizontal, 15);
    label_box.append(&name_label);
    label_box.append(&price_label);
    form_box.append(&label_box);
    
    let form_box_ref = Rc::new(RefCell::new(form_box));
    
    add_form_row(&form_box_ref.borrow());
    
    let add_button = Button::with_label("+");
    
    let form_box_ref_clone = Rc::clone(&form_box_ref);
    
    add_button.connect_clicked(move |_| {
        add_form_row(&form_box_ref_clone.borrow());
    });
    
    content_area.append(&add_button);
    
    dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
    dialog.add_button("Next", gtk4::ResponseType::Accept);
    
    dialog.connect_response(|dialog, response| {
        if response == gtk4::ResponseType::Accept {
            println!("Form submitted!");
            // Here you would collect data from all the entries
        }
        dialog.close();
    });
    
    dialog.present();
}

fn add_form_row(form_box: &Box) {
    let item_box = gtk4::Box::new(Orientation::Horizontal, 10);
    
    let name_entry = Entry::new();
    let price_entry = Entry::new();
    
    name_entry.set_hexpand(true);
    price_entry.set_hexpand(true);
    price_entry.set_input_purpose(gtk4::InputPurpose::Number);
    
    item_box.append(&name_entry);
    item_box.append(&price_entry);
    
    form_box.append(&item_box);
}
