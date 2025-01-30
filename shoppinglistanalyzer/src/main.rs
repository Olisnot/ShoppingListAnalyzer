mod custom_button;

use gtk4::*;
use gtk4::prelude::*;
use gtk4::{glib, Application};
use custom_button::CustomButton;

const APP_ID: &str = "org.gtk_rs.ShoppingListAnalyzer";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let grid = Grid::new();

    let button = CustomButton::with_label("Press me!");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);
    grid.attach(&button, 0, 0, 500, 700);

    let switch = Switch::new();
    switch.set_active(true);
    grid.attach(&switch, 0, 701, 200, 200);

    let calendar = Calendar::new();
    grid.attach(&calendar, 0, 901, 500, 500);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .child(&grid)
        .build();
    window.present();
}
