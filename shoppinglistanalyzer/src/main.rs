mod ui;
mod sqlite;

use gtk4::prelude::*;
use gtk4::{glib, Application};

const APP_ID: &str = "org.gtk_rs.ShoppingListAnalyzer";

fn main() -> glib::ExitCode {
    sqlite::start_database();
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}

