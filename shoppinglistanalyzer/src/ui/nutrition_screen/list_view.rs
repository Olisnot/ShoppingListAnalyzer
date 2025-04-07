use gtk4::*;
use gtk4::prelude::*;
use super::Nutrition;

impl Nutrition {
    pub fn create_list_view(&mut self) {
        let mut items: Vec<&str> = Vec::new();
        let db_items = self.database.borrow().get_items();

        for item in db_items.iter() {
            items.push(&item.name);
        }
        let string_list = StringList::new(&items);
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
        let selection_model = SingleSelection::new(Some(string_list));

        let list_view = ListView::new(Some(selection_model), Some(factory));

        let scrolled_window = ScrolledWindow::builder()
            .min_content_height(200)
            .vexpand(true)
            .hscrollbar_policy(PolicyType::Never)
            .child(&list_view)
            .build();

        self.item_selector = Some(scrolled_window);
    }
}
