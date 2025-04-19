mod ui;
mod sqlite;
mod data_structures;
mod categorization;

use gtk4::prelude::*;
use gtk4::{glib, Application};

const APP_ID: &str = "org.gtk_rs.ShoppingListAnalyzer";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}

