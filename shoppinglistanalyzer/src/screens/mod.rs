mod single_list_screen;
mod multi_list_screen;
mod nutrition_screen;

use gtk4::*;
use gtk4::prelude::*;
use gtk4::Application;

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
    let mut notebook = Notebook::new();

    let single_list = single_list_screen::create_single_list_screen();
    let multi_list = multi_list_screen::create_multi_list_screen();
    let nutrition_screen = nutrition_screen::create_nutrition_screen();

    notebook.create_tab("Single List", single_list.upcast());
    notebook.create_tab("Aggregated Lists", multi_list.upcast());
    notebook.create_tab("Nutrition", nutrition_screen.upcast());

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shopping List Analyzer")
        .child(&notebook.notebook)
        .build();
    window.present();
}
