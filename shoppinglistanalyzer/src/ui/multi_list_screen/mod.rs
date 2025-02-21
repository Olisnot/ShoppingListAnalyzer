use gtk4::*;
use gtk4::prelude::*;

pub fn create_multi_list_screen() -> Grid {
    let screen = Grid::new();
    let switch = Switch::new();
    screen.attach(&switch, 0, 0, 300, 400);
    screen
}
