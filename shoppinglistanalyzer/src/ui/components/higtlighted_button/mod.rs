use gtk4::{prelude::*, Button, Label};

pub fn create_highlighted_button(label_text: &str) -> Button {
    let button = Button::new();
    let label = Label::new(Some(label_text));
    button.set_child(Some(&label));
    button.set_size_request(100, 100);
    button.set_hexpand(false);
    button.set_vexpand(false);
    button.add_css_class("highlighted-button");
    button
}
