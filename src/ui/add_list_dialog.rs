use std::rc::Rc;
use std::cell::RefCell;

use gtk4::{
    ApplicationWindow,
    prelude::{GtkWindowExt, DialogExt, PopoverExt, WidgetExt, EditableExt, BoxExt, ButtonExt, ComboBoxExt, ComboBoxExtManual, EntryExt, Cast, CastNone},
    Box,
    ButtonsType,
    MessageType,
    MessageDialog,
    EntryCompletion,
    InputPurpose,
    ListStore,
    PolicyType,
    ScrolledWindow,
    Calendar,
    Button,
    Popover,
    Orientation,
    ResponseType,
    Dialog,
    Stack,
    Entry,
    ComboBoxText,
    glib::Type
};

use crate::sqlite::Database;
use crate::data_structures::*;
use crate::categorization::categorize;
use super::single_list_screen::SingleList;
use super::multi_list_screen::MultiList;
use super::items_screen::ItemsViewer;

pub type RowTuple = Rc<RefCell<Vec<(Entry, Entry, ComboBoxText)>>>;

pub fn show_add_list_dialog(parent: &ApplicationWindow, database: Rc<RefCell<Database>>, stack: Stack) {
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
    let date = calendar.date();
    date_button.borrow().set_label(&format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day_of_month()));

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

    let db_items = database.borrow().get_items(); 
    for item in db_items.iter() {
        store.borrow().set(&store.borrow().append(), &[(0, &item.name)]);
    }

    let rows: RowTuple = Rc::new(RefCell::new(Vec::new()));

    let first_row = build_form_row(Rc::clone(&store), &dialog, Rc::clone(&rows), Rc::clone(&database));
    form_box_ref.borrow().append(&first_row);
    
    let add_button_container = Rc::new(Box::new(Orientation::Horizontal, 0));
    add_button_container.set_hexpand(true);
    add_button_container.set_margin_top(5);
    add_button_container.set_margin_bottom(10);

    let add_button = Button::with_label("+");
    add_button.set_hexpand(true);
    add_button_container.append(&add_button);

    form_box_ref.borrow().append(&*add_button_container);
    
    let db_clone = Rc::clone(&database);
    let form_box_clone = Rc::clone(&form_box_ref);
    let store_clone = Rc::clone(&store);
    let dialog_clone2 = dialog.clone();
    let add_button_container = Rc::clone(&add_button_container);

    let rows_clone = Rc::clone(&rows);
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

    let categorization_button = Button::with_label("Categorize");
    let rows_clone_3 = Rc::clone(&rows);
    let db_clone = Rc::clone(&database);
    categorization_button.connect_clicked(move |_| {
        categorize(Rc::clone(&rows_clone_3), Rc::clone(&db_clone));
    });
    content_area.append(&categorization_button);
    
    let database_clone = Rc::clone(&database);
    let rows_clone_2 = Rc::clone(&rows);
    dialog.connect_response(move|dialog, response| {
        if response == ResponseType::Accept {
            let date_string: String = date_button.borrow().label().unwrap().to_string();
            parse_add_database(Rc::clone(&database_clone), date_string, Rc::clone(&rows_clone_2));
            refresh_stack(&stack, Rc::clone(&database_clone));
        }
        parent_clone.queue_draw();
        dialog.close();
    });
    dialog.present();

}

pub fn build_form_row(store: Rc<RefCell<ListStore>>, parent_dialog: &Dialog, rows: RowTuple, db: Rc<RefCell<Database>>) -> gtk4::Box {
    let item_box = Box::new(Orientation::Horizontal, 10);

    let remove_button = Button::with_label("✕");
    remove_button.set_tooltip_text(Some("Remove this item"));

    let name_entry = Entry::new();
    let price_entry = Entry::new();
    let category_combo = ComboBoxText::new();

    name_entry.set_hexpand(true);
    name_entry.set_placeholder_text(Some("Name"));
    price_entry.set_hexpand(true);
    price_entry.set_placeholder_text(Some("Price"));
    price_entry.set_input_purpose(InputPurpose::Number);

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
    category_combo.set_active(Some(0));

    let combo_clone = category_combo.clone();
    let existing_items = db.borrow().get_items();
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
    let rows_clone = Rc::clone(&rows);

    let row = Rc::new(RefCell::new((name_entry.clone(), price_entry.clone(), category_combo.clone())));
    let row_clone = Rc::clone(&row);
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
        let row_clone_2 = Rc::clone(&row_clone);

        let item_box_inner = item_box_clone.clone();
        confirm_dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Yes {
                if let Some(parent) = item_box_inner.parent() {
                    let index = rows_clone_2.borrow_mut().iter().position(|x| *x == *row_clone_2.borrow());
                    if let Some(index_unwraped) = index {
                        rows_clone_2.borrow_mut().remove(index_unwraped);
                        parent.downcast::<gtk4::Box>().unwrap().remove(&item_box_inner);
                    }
                }
            }
            dialog.close();
        });

        confirm_dialog.present();
    });

    rows.borrow_mut().push(row.borrow().clone());

    item_box.append(&remove_button);
    item_box.append(&name_entry);
    item_box.append(&price_entry);
    item_box.append(&category_combo);

    item_box
}

pub fn parse_add_database(database: Rc<RefCell<Database>>, date: String, rows: RowTuple) {
    let mut items: Vec<Item> = Vec::new();

    for (name, price, category) in rows.borrow_mut().iter() {
        items.push(Item::new(0, name.text().to_string(), category.active_text().unwrap().to_string(), price.text().to_string().parse().unwrap()));
    }

    let the_list: List = List::new(0, items, date);
    database.borrow().store_list(&the_list);

}

pub fn refresh_stack(stack: &Stack, database: Rc<RefCell<Database>>) {
    // Clear the stack
    while let Some(child) = stack.first_child() {
        stack.remove(&child);
    }

    let single_list = SingleList::new(database.clone());
    let single_list_grid = single_list.borrow_mut().create_single_list_screen();
    let multi_list = MultiList::new(database.clone());
    let multi_list_screen = multi_list.borrow_mut().create_multi_list_screen();
    let items = ItemsViewer::new(database.clone());
    let items_screen = items.borrow_mut().create_items_screen();

    let single_list_page = stack.add_named(&single_list_grid, Some("single_list"));
    single_list_page.set_title("Single List");
    let multi_list_page = stack.add_named(&multi_list_screen, Some("multi_list"));
    multi_list_page.set_title("Multi List");
    let items_screen_page = stack.add_named(&items_screen, Some("items_screen"));
    items_screen_page.set_title("Items");
}
