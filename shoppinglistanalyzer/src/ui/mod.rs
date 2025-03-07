mod single_list_screen;
mod multi_list_screen;
mod nutrition_screen;
mod add_list_dialog;

use gtk4::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4::gdk::Display;

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

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .build();

    let main_box = gtk4::Box::new(Orientation::Vertical, 0);
    
    let top_box = gtk4::Box::new(Orientation::Horizontal, 0);
    top_box.set_hexpand(true);

    let add_list_button = Button::with_label("Add List");
    let window_clone = window.clone();
    add_list_button.connect_clicked(move |_| {
        add_list_dialog::show_add_list_dialog(&window_clone);
    });
    top_box.append(&add_list_button);

    let mut notebook = Notebook::new();

    let single_list = single_list_screen::create_single_list_screen();
    let multi_list = multi_list_screen::create_multi_list_screen();
    let nutrition_screen = nutrition_screen::create_nutrition_screen();

    notebook.create_tab("Single List", single_list.upcast());
    notebook.create_tab("Aggregated Lists", multi_list.upcast());
    notebook.create_tab("Nutrition", nutrition_screen.upcast());

    main_box.append(&top_box);
    main_box.append(&notebook.notebook);

    window.set_child(Some(&main_box));

    let provider = CssProvider::new();
    provider.load_from_path("./components/style.css");

    let display = Display::default().unwrap();
    gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.present();
}
