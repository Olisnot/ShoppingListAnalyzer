use std::{rc::Rc, cell::RefCell};
use gtk4::*;
use gtk4::prelude::*;
use super::ItemsViewer;

impl ItemsViewer {
    pub fn create_list_view(&mut self) {
        let mut items_strs: Vec<&str> = Vec::new();

        for item in self.items.iter() {
            items_strs.push(&item.name);
        }
        self.string_list = Some(StringList::new(&items_strs));
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item.set_child(Some(&label));
        });

        factory.connect_bind(move |_, list_item| {
            let string_object = list_item
                .item()
                .and_then(|obj| obj.downcast::<StringObject>().ok())
                .expect("Failed to get StringObject");

            let label = list_item
                .child()
                .and_then(|w| w.downcast::<Label>().ok())
                .expect("Failed to get Label");

            label.set_label(&string_object.string());
        });
        let selection_model = Rc::new(RefCell::new(SingleSelection::new(self.string_list.clone())));

        let list_view = ListView::new(Some(selection_model.borrow().clone()), Some(factory));

        self.create_information_panel(0);

        let string_list_copy = self.string_list.as_ref().unwrap().clone();
        let self_rc = self.self_ref.as_ref().unwrap().clone();
        selection_model.borrow().connect_selected_notify(move |sel| {
            let pos = sel.selected();

            if pos != gtk4::INVALID_LIST_POSITION {
                if let Some(obj) = string_list_copy.item(pos) {
                    if obj.downcast::<StringObject>().is_ok() {
                        self_rc.borrow_mut().create_information_panel(pos.into());
                    }
                }
            } else {
                println!("No item selected");
            }
        });

        let scrolled_window = ScrolledWindow::builder()
            .min_content_height(200)
            .vexpand(true)
            .hscrollbar_policy(PolicyType::Never)
            .child(&list_view)
            .build();

        self.item_selector = Some(scrolled_window);
        self.main_content.as_ref().unwrap().set_start_child(self.item_selector.as_ref());
    }
}
