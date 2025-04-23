mod gemini;

use std::{rc::Rc, cell::RefCell};
use gtk4::prelude::EditableExt;
use crate::sqlite::*;
use crate::ui::add_list_dialog::RowTuple;
use gemini::categorize_new_items_with_gemini;

// Entry function for the categorization process responsible for splitting the item rows into new
// and existing items then starting the async function
pub fn categorize(mut items: RowTuple, db: Rc<RefCell<Database>>) {
    let db_items = db.borrow().get_items();
    let categorized_items: RowTuple = Rc::new(RefCell::new(Vec::new()));
    let new_items: RowTuple = Rc::new(RefCell::new(Vec::new()));

    for (name, _price, category) in items.borrow().iter() {
        let mut matched = false;
        for item in db_items.iter() {
            if item.name == name.text() {
                categorized_items.borrow_mut().push((name.clone(), _price.clone(), category.clone()));
                matched = true;
            }
        }

        if !matched {
            new_items.borrow_mut().push((name.clone(), _price.clone(), category.clone()));
        }
    }

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _result = runtime.block_on(categorize_new_items_with_gemini(&new_items, Rc::clone(&categorized_items)));

    items = categorized_items;
}
