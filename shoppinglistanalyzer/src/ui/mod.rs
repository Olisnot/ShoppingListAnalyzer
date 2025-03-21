mod single_list_screen;
mod multi_list_screen;
mod nutrition_screen;
mod add_list_dialog;

use std::rc::Rc;
use std::cell::RefCell;
use gtk4::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4::gdk::Display;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .build();

    let main_box = gtk4::Box::new(Orientation::Vertical, 5);
    
    let top_box = gtk4::Box::new(Orientation::Horizontal, 250);
    top_box.set_hexpand(true);

    let single_list = Rc::new(RefCell::new(single_list_screen::create_single_list_screen()));
    let multi_list = multi_list_screen::create_multi_list_screen();
    let nutrition_screen = nutrition_screen::create_nutrition_screen();

    let add_list_button = Button::with_label("Add List");
    let window_clone = window.clone();
    let single_list_clone = Rc::clone(&single_list);
    add_list_button.connect_clicked(move |_| {
        add_list_dialog::show_add_list_dialog(&window_clone, Rc::clone(&single_list_clone));
        let width = single_list_clone.borrow().width();
        let height = single_list_clone.borrow().height();
        println!("Grid size: {}x{}", width, height);
    });
    top_box.append(&add_list_button);

    let stack = Stack::new();
    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(&stack));

    let single_list_page = stack.add_named(&*single_list.borrow(), Some("single_list"));
    single_list_page.set_title("Single List");
    let multi_list_page = stack.add_named(&multi_list, Some("multi_list"));
    multi_list_page.set_title("Multi List");
    let nutrition_page = stack.add_named(&nutrition_screen, Some("nutrition_screen"));
    nutrition_page.set_title("Nutrition");


    main_box.append(&top_box);
    main_box.append(&switcher);
    main_box.append(&stack);

    window.set_child(Some(&main_box));

    let provider = CssProvider::new();
    provider.load_from_path("./components/style.css");

    let display = Display::default().unwrap();
    gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.present();
}
