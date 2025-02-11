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

struct Notebook {
    notebook: gtk4::Notebook,
    tabs: Vec<gtk4::Box>,
}

impl Notebook {
    fn new() -> Notebook {
        Notebook {
            notebook: gtk4::Notebook::new(),
            tabs: Vec::new(),
        }
    }

    fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let label = gtk4::Label::new(Some(title));
        let tab = gtk4::Box::new(Orientation::Horizontal, 0);
        tab.append(&label);
        let index = self.notebook.append_page(&widget, Some(&tab));
        self.tabs.push(tab);
        index
    }
}

fn build_ui(app: &Application) {
    let mut notebook = Notebook::new();
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

    let button2 = CustomButton::with_label("Press me!");
    button2.set_margin_top(12);
    button2.set_margin_bottom(12);
    button2.set_margin_start(12);
    button2.set_margin_end(12);

    notebook.create_tab("main", grid.upcast());
    notebook.create_tab("Not Main", button2.upcast());

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .child(&notebook.notebook)
        .build();
    window.present();
}
