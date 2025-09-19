use crate::ui::single_list_screen::pie_chart;
use gtk4::prelude::*;
use gtk4::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_total_list(store: Rc<RefCell<ListStore>>) -> Box {
    let mut total: f64 = 0.0;
    store.borrow().foreach(|_model, _path, iter| {
        let value = store.borrow().get::<f64>(iter, 2);
        total += value;
        false
    });

    let total_label = Rc::new(RefCell::new(Label::new(Some(&format!(
        "Total: {:.2}",
        total
    )))));
    total_label.borrow().set_halign(Align::Start);

    let sort_model = TreeModelSort::with_model(&*store.borrow());

    let tree_view = TreeView::new();
    tree_view.set_model(Some(&sort_model));

    let columns = ["Name", "Category", "Price"];
    for (i, title) in columns.iter().enumerate() {
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.set_title(title);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", i as i32);
        tree_view.append_column(&column);
    }

    sort_model.set_sort_column_id(SortColumn::Index(2), SortType::Descending);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&tree_view));
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let pie = pie_chart::draw(store);
    let list_pie_box = Paned::new(Orientation::Horizontal);
    list_pie_box.set_start_child(Some(&scrolled_window));
    list_pie_box.set_end_child(Some(&pie));
    list_pie_box.set_vexpand(true);
    list_pie_box.set_hexpand(true);
    list_pie_box.set_position(600);

    let list_box = Box::new(Orientation::Vertical, 15);
    list_box.set_vexpand(true);
    list_box.set_hexpand(true);
    list_box.append(&*total_label.borrow());
    list_box.append(&list_pie_box);
    list_box
}
