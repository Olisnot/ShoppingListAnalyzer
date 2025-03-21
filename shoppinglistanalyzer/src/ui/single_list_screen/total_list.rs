use std::rc::Rc;
use std::cell::RefCell;
use gtk4::*;
use gtk4::prelude::*;

pub fn create_total_list(store: Rc<RefCell<ListStore>>) -> Box {
    let tree_view = TreeView::new();
    tree_view.set_model(Some(&*store.borrow_mut()));

    let columns = ["Name", "Category", "Price"];
    for (i, title) in columns.iter().enumerate() {
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.set_title(title);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", i as i32);
        tree_view.append_column(&column);
    }

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&tree_view));
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let list_box = Box::new(Orientation::Vertical, 15);
    list_box.set_vexpand(true);
    list_box.set_hexpand(true);
    list_box.append(&scrolled_window);
    list_box
}
