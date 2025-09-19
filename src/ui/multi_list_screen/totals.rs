use std::{rc::Rc, cell::RefCell};
use gtk4::*;
use gtk4::prelude::*;

pub fn create_totals(store: Rc<RefCell<TreeStore>>) -> Box{
    let tree_box = Box::new(Orientation::Vertical, 15);
    tree_box.set_vexpand(true);
    tree_box.set_hexpand(true);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let tree_view = TreeView::with_model(&*store.borrow());

    let name_renderer = CellRendererText::new();
    let name_column = TreeViewColumn::new();
    name_column.set_title("Name");
    name_column.pack_start(&name_renderer, true);
    name_column.add_attribute(&name_renderer, "text", 0);
    tree_view.append_column(&name_column);

    let category_renderer = CellRendererText::new();
    let category_column = TreeViewColumn::new();
    category_column.set_title("Category");
    category_column.pack_start(&category_renderer, true);
    category_column.add_attribute(&category_renderer, "text", 1);
    tree_view.append_column(&category_column);

    let price_renderer = CellRendererText::new();
    let price_column = TreeViewColumn::new();
    price_column.set_title("Price");
    price_column.pack_start(&price_renderer, true);
    price_column.add_attribute(&price_renderer, "text", 2);
    tree_view.append_column(&price_column);
    scrolled_window.set_child(Some(&tree_view));
    tree_box.append(&scrolled_window);
    tree_box
}
