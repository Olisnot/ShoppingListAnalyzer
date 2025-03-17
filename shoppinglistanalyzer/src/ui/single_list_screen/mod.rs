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

    let text_view = TextView::new();
    text_view.set_editable(false);
    let buffer = text_view.buffer();
    fill_items_text_buffer(list_id, buffer.clone());
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&text_view));
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);
    screen.attach(&scrolled_window, 0, 1000, 1000, 1000);


    let buffer_clone = buffer.clone();
    list_selector.connect_changed(move |list|{
        buffer_clone.set_text("");
        let list_id = extract_first_number(&list.active_text().unwrap()).unwrap();
        fill_items_text_buffer(list_id, buffer_clone.clone());
    });

    screen.attach(&list_selector, 0, 0, 500, 300);


    screen
}

fn extract_first_number(s: &str) -> Option<i64> {
    // Find the first sequence of digits in the string
    let number_str = s.chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    
    // Parse the extracted digits into an i64
    if number_str.is_empty() {
        None
    } else {
        number_str.parse::<i64>().ok()
    }
}

fn fill_items_text_buffer(list_id: i64, buffer: TextBuffer) {
    let items = sqlite::get_items(list_id);
    let text_view = TextView::new();
    text_view.set_editable(false);
    let mut end = buffer.end_iter();
    for (i, string) in items.iter().enumerate() {
        if i > 0 {
            buffer.insert(&mut end, "\n");
        }
        buffer.insert(&mut end, string);
    }
}
