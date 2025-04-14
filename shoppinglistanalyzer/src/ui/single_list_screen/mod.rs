mod total_list;
mod category_list;
mod pie_chart;
mod edit_list_dialog;

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
    active_list_id: i64,
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
            active_list_id: 0,
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

        if !lists.is_empty() {
            self.list_selector.set_active(Some(length-1));
            self.active_list_id = extract_first_number(&self.list_selector.active_text().unwrap()).unwrap();
        }

        self.fill_items(Rc::clone(&self.store));

        self.total_list = Some(total_list::create_total_list(Rc::clone(&self.store))); 
        self.category_list = Some(category_list::create_category_list(Rc::clone(&self.store)));

        screen.attach(self.total_list.as_ref().unwrap(), 0, 1, 1, 1);
        screen.attach(self.category_list.as_ref().unwrap(), 0, 2, 2, 2);

        let edit_list_button = Button::with_label("Edit");
        screen.attach(&edit_list_button, 1, 0, 1, 1);

        let self_rc = self.self_ref.as_ref().unwrap().clone();
        let self_rc_clone = Rc::clone(&self_rc);
        edit_list_button.connect_clicked(move |button| {
            if let Some(window) = button.root().and_then(|w| w.downcast::<ApplicationWindow>().ok()) {
                self_rc.borrow().show_edit_list_dialog(&window);
            }
        });

        let store_clone = Rc::clone(&self.store);
        let screen_clone = screen.clone();
        self.list_selector.connect_changed(move |list|{
            store_clone.borrow_mut().clear();
            if list.active().is_some() {
                self_rc_clone.borrow_mut().active_list_id = extract_first_number(&list.active_text().unwrap()).unwrap();
                self_rc_clone.borrow().fill_items(store_clone.clone());
                self_rc_clone.borrow_mut().refresh_ui(screen_clone.clone());
            }
        });
        screen.attach(&self.list_selector, 0, 0, 1, 1);

        screen
    }

    pub fn refresh_ui(&mut self, screen: Grid) {
        screen.remove(self.total_list.as_ref().unwrap());
        screen.remove(self.category_list.as_ref().unwrap());
        self.total_list = Some(total_list::create_total_list(Rc::clone(&self.store)));
        self.category_list = Some(category_list::create_category_list(Rc::clone(&self.store)));
        screen.attach(self.total_list.as_ref().unwrap(), 0, 1, 1, 1);
        screen.attach(self.category_list.as_ref().unwrap(), 0, 2, 2, 2);
    }

    fn fill_items(&self, store: Rc<RefCell<ListStore>>) {
        if self.active_list_id == 0 {
            return;
        }
        let items = self.database.borrow().get_items_by_list_id(self.active_list_id);
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

