mod totals;
mod categories;
mod charts;

use std::{rc::Rc, cell::RefCell};
use gtk4::glib::DateTime;
use gtk4::*;
use gtk4::prelude::*;
use crate::sqlite::Database;
use totals::create_totals;
use categories::create_categories;
use charts::create_charts;

pub struct MultiList {
    store: Rc<RefCell<TreeStore>>,
    database: Rc<RefCell<Database>>,
    self_ref: Option<Rc<RefCell<MultiList>>>,
    start_date_calendar: Calendar,
    end_date_calendar: Calendar,
    start_date_selector: Option<Button>,
    end_date_selector: Option<Button>,
    totals: Option<Box>,
    categories: Option<Box>,
    charts: Option<Box>,
    main_content: Option<Paned>,
}
impl MultiList {
    pub fn new(db: Rc<RefCell<Database>>) -> Rc<RefCell<Self>> {
        let store = Rc::new(RefCell::new(TreeStore::new(&[String::static_type(), String::static_type(), f64::static_type()])));
        let list = Rc::new(RefCell::new(MultiList {
            store: Rc::clone(&store),
            database: db,
            start_date_calendar: Calendar::new(),
            end_date_calendar: Calendar::new(),
            start_date_selector: None,
            end_date_selector: None,
            totals: None,
            categories: None,
            charts: None,
            main_content: None,
            self_ref: None,
        }));
        list.borrow_mut().self_ref = Some(Rc::clone(&list));
        list
    }

    pub fn create_multi_list_screen(&mut self) -> Box {
        let screen = Box::new(Orientation::Vertical, 15);
        screen.set_vexpand(true);
        screen.set_hexpand(true);

        let start_date_popover = Popover::new();
        start_date_popover.set_has_arrow(false);
        start_date_popover.set_child(Some(&self.start_date_calendar));

        let end_date_popover = Popover::new();
        end_date_popover.set_has_arrow(false);
        end_date_popover.set_child(Some(&self.end_date_calendar));

        let date_selectors_box = Box::new(Orientation::Horizontal, 15);
        self.start_date_selector = Some(self.create_date_button(&self.start_date_calendar));
        self.end_date_selector = Some(self.create_date_button(&self.end_date_calendar));
        date_selectors_box.append(self.start_date_selector.as_ref().unwrap());
        date_selectors_box.append(self.end_date_selector.as_ref().unwrap());
        screen.append(&date_selectors_box);

        start_date_popover.set_parent(self.start_date_selector.as_ref().unwrap());
        let start_date_popover_clone = start_date_popover.clone();
        self.start_date_selector.as_ref().unwrap().connect_clicked(move |_| {
            start_date_popover_clone.popup();
        });

        let start_date_selector_clone = self.start_date_selector.as_ref().unwrap().clone();
        let start_date_popover_clone_2 = start_date_popover.clone();
        if let Some(calendar) = start_date_popover.child().and_downcast::<Calendar>() {
            calendar.connect_day_selected(move |cal| {
                let date = cal.date();
                start_date_selector_clone.set_label(&format!("{}-{}-{}", date.day_of_month(), date.month(), date.year()));
                start_date_popover_clone_2.popdown();
            });
        }

        end_date_popover.set_parent(self.end_date_selector.as_ref().unwrap());
        let end_date_popover_clone = end_date_popover.clone();
        self.end_date_selector.as_ref().unwrap().connect_clicked(move |_| {
            end_date_popover_clone.popup();
        });

        let end_date_selector_clone = self.end_date_selector.as_ref().unwrap().clone();
        let end_date_popover_clone_2 = end_date_popover.clone();
        if let Some(calendar) = end_date_popover.child().and_downcast::<Calendar>() {
            calendar.connect_day_selected(move |cal| {
                let date = cal.date();
                end_date_selector_clone.set_label(&format!("{}-{}-{}", date.day_of_month(), date.month(), date.year()));
                end_date_popover_clone_2.popdown();
            });
        }

        self.create_main_content(&self.start_date_calendar.date(), &self.end_date_calendar.date());

        screen.append(self.main_content.as_ref().unwrap());

        let calculate_button = Button::with_label("Calculate");
        let screen_clone = screen.clone();
        let start_calendar_clone = self.start_date_calendar.clone();
        let end_calendar_clone = self.end_date_calendar.clone();
        let self_rc = self.self_ref.as_ref().unwrap().clone();
        calculate_button.connect_clicked(move |_|{
            self_rc.borrow().populate_store(&start_calendar_clone.date(), &end_calendar_clone.date());
            self_rc.borrow_mut().refresh_ui(screen_clone.clone(), &start_calendar_clone.date(), &end_calendar_clone.date());
        });
        date_selectors_box.append(&calculate_button);

        screen
    }

    fn create_date_button(&self, calendar: &Calendar) -> Button {
        let date = calendar.date();
        Button::with_label(&format!("{}-{}-{}", date.day_of_month(), date.month(), date.year()))
    }

    fn create_main_content(&mut self, start_date: &DateTime, end_date: &DateTime) {
        let lists = self.database.borrow().get_lists_in_dates_range(start_date, end_date);

        let main_content = Paned::new(Orientation::Horizontal);
        main_content.set_vexpand(true);
        main_content.set_hexpand(true);

        let totals_category_box = Box::new(Orientation::Vertical, 15);
        totals_category_box.set_vexpand(true);
        totals_category_box.set_hexpand(true);

        let totals_labels_box = Box::new(Orientation::Horizontal, 12);
        let mut total = 0.0;
        for list in lists.iter() {
            total += list.get_total_cost();
        }
        let average = total/lists.len() as f64;
        let total_label = Label::new(Some(&format!("Total: {:.2}", total)));
        let average_cost_label = Label::new(Some(&format!("Average Cost: {:.2}", average)));
        totals_labels_box.append(&total_label);
        totals_labels_box.append(&average_cost_label);
        totals_category_box.append(&totals_labels_box);

        self.totals = Some(create_totals(Rc::clone(&self.store)));
        totals_category_box.append(self.totals.as_ref().unwrap());


        self.categories = Some(create_categories(Rc::clone(&self.store)));
        totals_category_box.append(self.categories.as_ref().unwrap());

        main_content.set_start_child(Some(&totals_category_box));

        let lists_rc = Rc::new(RefCell::new(lists));
        self.charts = Some(create_charts(Rc::clone(&self.store), Rc::clone(&lists_rc)));
        main_content.set_end_child(Some(self.charts.as_ref().unwrap()));

        main_content.set_position(600);

        self.main_content = Some(main_content);
    }

    fn populate_store(&self, start_date: &DateTime, end_date: &DateTime) {
        self.store.borrow().clear();
        let lists = self.database.borrow().get_lists_in_dates_range(start_date, end_date);
        for list in lists.iter() {
            for item in list.items.iter() {
                let mut item_added = false;
                let parent: TreeIter;
                if let Some(iter) = self.store.borrow().iter_first() {
                    if self.store.borrow().iter_is_valid(&iter) {
                        loop {
                            let value: Option<String> = if self.store.borrow().iter_is_valid(&iter) {
                                Some(self.store.borrow().get::<String>(&iter, 0))
                            } else {
                                None
                            };

                            if let Some(value) = value{
                                if value.to_lowercase() == item.name.to_lowercase() {
                                    item_added = true;
                                    parent = iter;
                                    break;
                                }
                                if !self.store.borrow().iter_next(&iter) {
                                    parent = self.store.borrow().append(None);
                                    break;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                else {
                    parent = self.store.borrow().append(None);
                }
                if !item_added {
                    self.store.borrow().set(&parent, &[(0, &item.name), (1, &item.category), (2, &item.price)]);
                }
                let child = self.store.borrow().append(Some(&parent));
                self.store.borrow().set(&child, &[(0, &format!("({}) {}", list.id, list.date)), (1, &item.category), (2, &item.price)]);
            }
        }
        if let Some(iter) = self.store.borrow().iter_first() {
            loop {
                self.sum_child_prices(&iter);
                if !self.store.borrow().iter_next(&iter) {
                    break;
                }
            }
        }
    }

    fn sum_child_prices(&self, parent: &TreeIter) {
        let mut total_price = 0.0;

        if let Some(child_iter) = self.store.borrow().iter_children(Some(parent)) {
            loop {
                let child_price: f64 = self.store.borrow().get::<f64>(&child_iter, 2);
                total_price += child_price;

                if !self.store.borrow().iter_next(&child_iter) {
                    break;
                }
            }
        }

        self.store.borrow().set_value(parent, 2, &total_price.to_value());
    }

    pub fn refresh_ui(&mut self, screen: Box, start_date: &DateTime, end_date: &DateTime) {
        screen.remove(self.main_content.as_ref().unwrap());
        self.categories = Some(create_categories(Rc::clone(&self.store)));
        self.create_main_content(start_date, end_date);
        screen.append(self.main_content.as_ref().unwrap());
    }
}
