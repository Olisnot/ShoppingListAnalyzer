use std::{rc::Rc, cell::RefCell};
use gtk4::*;
use gtk4::prelude::*;
use crate::data_structures::Categories;

pub fn create_categories(store: Rc<RefCell<TreeStore>>) -> Box{
    let tree_box = Box::new(Orientation::Horizontal, 15);
    tree_box.set_vexpand(true);
    tree_box.set_hexpand(true);

    let stack = Stack::new();
    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(&stack));
    switcher.set_orientation(Orientation::Vertical);

    let protein_tree = create_tree_view(Rc::clone(&store), Categories::Protein.to_cat_string());
    let fruits_vegetables_tree = create_tree_view(Rc::clone(&store), Categories::FruitsVegetables.to_cat_string());
    let dairy_tree = create_tree_view(Rc::clone(&store), Categories::Dairy.to_cat_string());
    let carbohydrates_tree = create_tree_view(Rc::clone(&store), Categories::Carbohydrates.to_cat_string());
    let fats_oils_tree = create_tree_view(Rc::clone(&store), Categories::FatsOils.to_cat_string());
    let unhealthy_tree = create_tree_view(Rc::clone(&store), Categories::Unhealthy.to_cat_string());
    let hygiene_tree = create_tree_view(Rc::clone(&store), Categories::Hygiene.to_cat_string());
    let misc_tree = create_tree_view(Rc::clone(&store), Categories::Misc.to_cat_string());

    let protein_window = create_scrolled_window(&protein_tree);
    let fruits_vegetables_window = create_scrolled_window(&fruits_vegetables_tree);
    let dairy_window = create_scrolled_window(&dairy_tree);
    let carbohydrates_window = create_scrolled_window(&carbohydrates_tree);
    let fats_oils_window = create_scrolled_window(&fats_oils_tree);
    let unhealthy_window = create_scrolled_window(&unhealthy_tree);
    let hygiene_window = create_scrolled_window(&hygiene_tree);
    let misc_window = create_scrolled_window(&misc_tree);

    let proteins_page = stack.add_named(&protein_window, Some("Proteins"));
    proteins_page.set_title("Proteins");

    let fruits_vegetables_page = stack.add_named(&fruits_vegetables_window, Some("Fruits/Vegetables"));
    fruits_vegetables_page.set_title("Fruits/Vegetables");

    let dairy_page = stack.add_named(&dairy_window, Some("Dairy"));
    dairy_page.set_title("Dairy");

    let carbohydrates_page = stack.add_named(&carbohydrates_window, Some("Carbohydrates"));
    carbohydrates_page.set_title("Carbohydrates");

    let fats_oils_page = stack.add_named(&fats_oils_window, Some("Fats/Oils"));
    fats_oils_page.set_title("Fats/Oils");

    let unhealthy_page = stack.add_named(&unhealthy_window, Some("Unhealthy"));
    unhealthy_page.set_title("Unhealthy");

    let hygiene_page = stack.add_named(&hygiene_window, Some("Hygiene"));
    hygiene_page.set_title("Hygiene");

    let misc_page = stack.add_named(&misc_window, Some("Miscellaneous"));
    misc_page.set_title("Miscellaneous");

    tree_box.append(&switcher);
    tree_box.append(&stack);
    tree_box
}

fn create_tree_view(store: Rc<RefCell<TreeStore>>, category_filter: String) -> Box {
    let tree_box = Box::new(Orientation::Vertical, 15);
    tree_box.set_vexpand(true);
    tree_box.set_hexpand(true);

    let mut total: f64 = 0.0;
    store.borrow().foreach(|_model, _path, iter| {
        let category = store.borrow().get::<String>(iter, 1);
        if category == category_filter {
            let value = store.borrow().get::<f64>(iter, 2);
            total += value;
        }
        false 
    });

    let total_label = gtk4::Label::new(Some(&format!("Total: {:.2}", total/2.0)));
    total_label.set_halign(Align::Start);
    tree_box.append(&total_label);

    let tree_view = TreeView::new();

    let filter_model = TreeModelFilter::new(&*store.borrow(), None);

    filter_model.set_visible_func(move |model, iter| {
        let value = model.get_value(iter, 1);
        match value.get::<String>() {
            Ok(category) => category == category_filter,
            _ => false, 
        }
    });


    tree_view.set_model(Some(&filter_model));

    let columns = ["Name", "", "Price"];
    for (i, title) in columns.iter().enumerate() {
        if i != 1 {
            let renderer = CellRendererText::new();
            let column = TreeViewColumn::new();
            column.set_title(title);
            column.pack_start(&renderer, true);
            column.add_attribute(&renderer, "text", i as i32);
            tree_view.append_column(&column);
        }
    }
    tree_box.append(&tree_view);
    tree_box
}

fn create_scrolled_window(tree_view: &Box) -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);
    scrolled_window.set_child(Some(tree_view));
    scrolled_window
}
