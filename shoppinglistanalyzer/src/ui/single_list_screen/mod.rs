#[path = "../../sqlite/mod.rs"]
mod sqlite;
#[path = "../components/higtlighted_button/mod.rs"]
mod higtlighted_button;

use gtk4::*;
use gtk4::prelude::*;

pub fn create_single_list_screen() -> Grid {
    let screen = Grid::new();
// add list button
    let add_list_button = higtlighted_button::create_highlighted_button("Add List");
    screen.attach(&add_list_button, 10, 10, 500, 250);

// list items
    let items = sqlite::get_items();
    let text_view = TextView::new();
    text_view.set_editable(false);
    let buffer = text_view.buffer();
    let mut end = buffer.end_iter();
    for (i, string) in items.iter().enumerate() {
        if i > 0 {
            buffer.insert(&mut end, "\n");
        }
        buffer.insert(&mut end, string);
    }
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&text_view));
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);
    screen.attach(&scrolled_window, 0, 1000, 1000, 1000);

    screen
}
