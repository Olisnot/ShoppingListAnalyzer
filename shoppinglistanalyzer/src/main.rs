mod custom_button;

use gtk4::*;
use gtk4::prelude::*;
use gtk4::{glib, Application};
use std::cell::*;

const APP_ID: &str = "org.gtk_rs.ShoppingListAnalyzer";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let is_clicked = Cell::new(false);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        if !is_clicked.get() {
            button.set_label("Hello World!");
        } else {
            button.set_label("Press me!");
        }
        is_clicked.set(!is_clicked.get());
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .child(&button)
        .build();
    // Present window

    window.present();
}
