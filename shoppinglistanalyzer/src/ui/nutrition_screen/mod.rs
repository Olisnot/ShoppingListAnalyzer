mod list_view;

use std::{rc::Rc, cell::RefCell};
use gtk4::*;
use gtk4::prelude::*;
use crate::sqlite::Database;

pub struct Nutrition {
    store: Rc<RefCell<TreeStore>>,
    database: Rc<RefCell<Database>>,
    main_content: Option<Paned>,
    item_selector: Option<ScrolledWindow>,
    information_panel: Option<Box>,
}

impl Nutrition {
    pub fn new(db: Rc<RefCell<Database>>) -> Rc<RefCell<Self>> {
        let store = Rc::new(RefCell::new(TreeStore::new(&[String::static_type(), String::static_type(), f64::static_type()])));
        let nutr = Rc::new(RefCell::new(Nutrition {
            store,
            main_content: None,
            item_selector: None,
            information_panel: None,
            database: db,
        }));
        nutr
    }

    pub fn create_nutrition_screen(&mut self) -> Box {
        let screen = Box::new(Orientation::Vertical, 10);
        self.create_main_content();
        screen.append(self.main_content.as_ref().unwrap());
        screen
    }
    
    fn create_main_content(&mut self) {
        let main_content = Paned::new(Orientation::Vertical);
        self.create_list_view();
        main_content.set_start_child(self.item_selector.as_ref());

        self.main_content = Some(main_content);
    }
}
