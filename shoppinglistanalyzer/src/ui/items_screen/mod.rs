mod list_view;
mod information_panel;

use std::{rc::Rc, cell::RefCell};
use gtk4::*;
use gtk4::prelude::*;
use crate::{data_structures::Item ,sqlite::Database};

pub struct ItemsViewer {
    string_list: Option<StringList>,
    database: Rc<RefCell<Database>>,
    items: Vec<Item>,
    main_content: Option<Paned>,
    item_selector: Option<ScrolledWindow>,
    information_panel: Option<Box>,
    self_ref: Option<Rc<RefCell<ItemsViewer>>>,
}

impl ItemsViewer {
    pub fn new(db: Rc<RefCell<Database>>) -> Rc<RefCell<Self>> {
        let items_list = db.borrow().get_items_in_lists();
        let item_screen = Rc::new(RefCell::new(ItemsViewer {
            string_list: None,
            items: items_list,
            main_content: None,
            item_selector: None,
            information_panel: None,
            database: db,
            self_ref: None,
        }));
        item_screen.borrow_mut().self_ref = Some(Rc::clone(&item_screen));
        item_screen
    }

    pub fn create_items_screen(&mut self) -> Box {
        let screen = Box::new(Orientation::Vertical, 10);
        self.create_main_content();
        screen.append(self.main_content.as_ref().unwrap());
        screen
    }

    fn create_main_content(&mut self) {
        self.main_content = Some(Paned::new(Orientation::Horizontal));
        self.main_content.as_ref().unwrap().set_position(200);
        self.create_list_view();
    }

    pub fn refresh_items(&mut self) {
        self.items = self.database.borrow().get_items_in_lists();
    }
}
