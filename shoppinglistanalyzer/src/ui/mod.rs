mod single_list_screen;
mod multi_list_screen;
mod items_screen;
mod add_list_dialog;

use std::cell::RefCell;
use std::rc::Rc;
use gtk4::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4::gdk::Display;
use single_list_screen::SingleList;
use multi_list_screen::MultiList;
use items_screen::ItemsViewer;
use crate::sqlite::Database;

pub fn build_ui(app: &Application) {
    let database = Rc::new(RefCell::new(Database::new()));
    database.borrow_mut().start_database();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .default_width(1280)
        .default_height(720)
        .build();

    let main_box = gtk4::Box::new(Orientation::Vertical, 5);

    let top_box = gtk4::Box::new(Orientation::Horizontal, 250);
    top_box.set_hexpand(true);

    let list_store = StringList::new(&[]);
    let lists = database.borrow().get_lists_dates();
    for list in lists.iter() {
        list_store.append(list);
    }

    let stack = Stack::new();
    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(&stack));

    let single_list = SingleList::new(database.clone());
    let single_list_grid = single_list.borrow_mut().create_single_list_screen();
    let multi_list = MultiList::new(database.clone());
    let multi_list_screen = multi_list.borrow_mut().create_multi_list_screen();
    let nutrition = ItemsViewer::new(database.clone());
    let nutrition_screen = nutrition.borrow_mut().create_nutrition_screen();

    add_to_stack(&stack, &single_list_grid, &multi_list_screen, &nutrition_screen);

    let add_list_button = Button::with_label("Add List");
    let window_clone = window.clone();
    let database_clone = Rc::clone(&database);
    let stack_clone = stack.clone();
    add_list_button.connect_clicked(move |_| {
        add_list_dialog::show_add_list_dialog(&window_clone, Rc::clone(&database_clone), stack_clone.clone());
    });
    top_box.append(&add_list_button);

    main_box.append(&top_box);
    main_box.append(&switcher);
    main_box.append(&stack);

    window.set_child(Some(&main_box));

    let css = CssProvider::new();
    css.load_from_data("
        #info-panel-label { font-size: 24px; } 
        #price-label { font-size: 20px; }
        #bold-label { font-weight: bold; }
        ");

    let display = Display::default().expect("Failed to get default display");
    style_context_add_provider_for_display(
        &display,
        &css,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );


    window.present();
}

fn add_to_stack(stack: &Stack, single_list: &Grid, multi_list: &Box, nutrition: &Box) {
    let single_list_page = stack.add_named(single_list, Some("single_list"));
    single_list_page.set_title("Single List");
    let multi_list_page = stack.add_named(multi_list, Some("multi_list"));
    multi_list_page.set_title("Multiple Lists");
    let nutrition_page = stack.add_named(nutrition, Some("nutrition_screen"));
    nutrition_page.set_title("Items");
}

