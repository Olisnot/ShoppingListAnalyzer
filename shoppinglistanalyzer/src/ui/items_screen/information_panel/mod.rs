mod dialogs;
mod chart;

use gtk4::{*, prelude::*, gdk::Display};
use super::ItemsViewer;

impl ItemsViewer {
    pub fn create_information_panel(&mut self, item_id: i64) {
        let info_box = Box::new(Orientation::Vertical, 10);
        let label = Label::new(Some(&self.items[item_id as usize].name));
        label.set_widget_name("info-panel-label");

        let css = CssProvider::new();
        css.load_from_data("#info-panel-label { font-size: 24px; }");

        let display = Display::default().expect("Failed to get default display");
        style_context_add_provider_for_display(
            &display,
            &css,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let header = HeaderBar::builder()
            .title_widget(&label)
            .show_title_buttons(false)
            .build();
        let edit_item_button = Button::with_label("Edit");
        let get_nutrition_button = Button::with_label("Get Nutrition");
        let button_box = Box::new(Orientation::Horizontal, 10);
        button_box.append(&edit_item_button);
        button_box.append(&get_nutrition_button);
        header.pack_start(&button_box);

        let self_rc = self.self_ref.as_ref().unwrap().clone();
        edit_item_button.connect_clicked(move |button| {
            if let Some(window) = button.root().and_then(|w| w.downcast::<ApplicationWindow>().ok()) {
                self_rc.borrow().create_edit_item_dialog(&window, item_id);
            }

        });

        info_box.append(&header);
        info_box.append(&self.draw_line_chart(self.items[item_id as usize].id));

        self.information_panel = Some(info_box);
        self.main_content.as_ref().unwrap().set_end_child(self.information_panel.as_ref());
    }
}
