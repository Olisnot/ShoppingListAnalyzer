mod total_list;
mod category_list;
mod pie_chart;

use std::rc::Rc;
use std::cell::RefCell;
use gtk4::*;
use gtk4::prelude::*;
use crate::sqlite;

pub fn create_single_list_screen() -> Grid {
    let screen = Grid::new();

    let list_selector = ComboBoxText::new();
    let lists = sqlite::get_lists_dates();
    for list_date in lists.iter() {
        list_selector.append(Some(list_date), list_date);
    }
    let length: u32 = lists.len().try_into().unwrap();
    list_selector.set_active(Some(length-1));

    let list_id: i64 = extract_first_number(&list_selector.active_text().unwrap()).unwrap();

    let store = Rc::new(RefCell::new(ListStore::new(&[String::static_type(), String::static_type(), f64::static_type()])));

    fill_items(list_id, Rc::clone(&store));

    let total_list = Rc::new(RefCell::new(total_list::create_total_list(Rc::clone(&store)))); 
    let category_list = Rc::new(RefCell::new(category_list::create_category_list(Rc::clone(&store), list_id))); 

    screen.attach(&*total_list.borrow(), 0, 1, 1, 1);
    screen.attach(&*category_list.borrow(), 0, 2, 1, 1);

    let store_clone = Rc::clone(&store);
    list_selector.connect_changed(move |list|{
        store_clone.borrow_mut().clear();
        let list_id = extract_first_number(&list.active_text().unwrap()).unwrap();
        fill_items(list_id, store_clone.clone());
    });

    screen.attach(&list_selector, 0, 0, 1, 1);


    screen
}

fn extract_first_number(s: &str) -> Option<i64> {
    let number_str = s.chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    
    if number_str.is_empty() {
        None
    } else {
        number_str.parse::<i64>().ok()
    }
}

fn fill_items(list_id: i64, store: Rc<RefCell<ListStore>>) {
    let items = sqlite::get_items(list_id);
    let text_view = TextView::new();
    text_view.set_editable(false);
    for item in items.iter() {
        add_row(Rc::clone(&store), &item.name, &item.category, item.price);
    }
}

fn add_row(store: Rc<RefCell<ListStore>>, name: &str, category: &str, price: f64) {
    let iter = store.borrow_mut().append();
    store.borrow_mut().set_value(&iter, 0, &name.to_value());
    store.borrow_mut().set_value(&iter, 1, &category.to_value());
    store.borrow_mut().set_value(&iter, 2, &price.to_value());
}
