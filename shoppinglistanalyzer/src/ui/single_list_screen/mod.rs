mod total_list;
mod category_list;
mod pie_chart;

use std::rc::Rc;
use std::cell::RefCell;
use gtk4::*;
use gtk4::prelude::*;
use crate::sqlite::Database;

pub struct SingleList {
    pub list_selector: ComboBoxText,
    pub total_list: Option<Box>,
    pub category_list: Option<Box>,
    store: Rc<RefCell<ListStore>>,
    database: Rc<RefCell<Database>>,
    self_ref: Option<Rc<RefCell<SingleList>>>,
}

impl SingleList {
    pub fn new(db: Rc<RefCell<Database>>) -> Rc<RefCell<Self>> {
        let store = Rc::new(RefCell::new(ListStore::new(&[String::static_type(), String::static_type(), f64::static_type()])));
        let list = Rc::new(RefCell::new(SingleList {
            store: Rc::clone(&store),
            list_selector: ComboBoxText::new(),
            total_list: None,
            category_list: None,
            database: db,
            self_ref: None,
        }));
        list.borrow_mut().self_ref = Some(Rc::clone(&list));

        list
    }

    pub fn create_single_list_screen(&mut self) -> Grid {
        let screen = Grid::new();

        let lists = self.database.borrow().get_lists_dates();
        for list_date in lists.iter() {
            self.list_selector.append(Some(list_date), list_date);
        }
        let length: u32 = lists.len().try_into().unwrap();

        let mut list_id: i64 = 0;

        if !lists.is_empty() {
            self.list_selector.set_active(Some(length-1));
            list_id = extract_first_number(&self.list_selector.active_text().unwrap()).unwrap();
        }

        self.fill_items(list_id, Rc::clone(&self.store));

        self.total_list = Some(total_list::create_total_list(Rc::clone(&self.store))); 
        self.category_list = Some(category_list::create_category_list(Rc::clone(&self.store)));

        screen.attach(self.total_list.as_ref().unwrap(), 0, 1, 1, 1);
        screen.attach(self.category_list.as_ref().unwrap(), 0, 2, 2, 2);

        let store_clone = Rc::clone(&self.store);
        let self_rc = self.self_ref.as_ref().unwrap().clone();
        self.list_selector.connect_changed(move |list|{
            store_clone.borrow_mut().clear();
            if list.active().is_some() {
                let list_id = extract_first_number(&list.active_text().unwrap()).unwrap();
                self_rc.borrow().fill_items(list_id, store_clone.clone());
            }
        });

        screen.attach(&self.list_selector, 0, 0, 1, 1);

        screen
    }

    pub fn refresh_selector(&self) {
        println!("refreshing selector");
        self.list_selector.set_active(None);
        self.list_selector.clear();
        let lists = self.database.borrow().get_lists_dates();
        for list_date in lists.iter() {
            self.list_selector.append(Some(list_date), list_date);
        }
        let length: u32 = lists.len().try_into().unwrap();
        if !lists.is_empty() {
            self.list_selector.set_active(Some(length-1));
        }
    }


    fn fill_items(&self, list_id: i64, store: Rc<RefCell<ListStore>>) {
        if list_id == 0 {
            return;
        }
        let items = self.database.borrow().get_items(list_id);
        let text_view = TextView::new();
        text_view.set_editable(false);
        for item in items.iter() {
            Self::add_row(Rc::clone(&store), &item.name, &item.category, item.price);
        }
    }

    fn add_row(store: Rc<RefCell<ListStore>>, name: &str, category: &str, price: f64) {
        let iter = store.borrow_mut().append();
        store.borrow_mut().set_value(&iter, 0, &name.to_value());
        store.borrow_mut().set_value(&iter, 1, &category.to_value());
        store.borrow_mut().set_value(&iter, 2, &price.to_value());
    }
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
