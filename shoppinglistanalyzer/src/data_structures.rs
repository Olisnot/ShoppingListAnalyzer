use std::cell::Cell;
use chrono;

enum Categories {
    Meat,
    Fish,
    Fruit,
    Vegetables,
    Dairy,
    Carbohydrates,
    Confectionary,
    Hygiene,
    Miscellaneous,
}

impl Categories {
    fn enum_to_string(&self) -> String {
        match self {
            Categories::Meat => "Meat".to_string(),
            Categories::Fish => "Fish".to_string(),
            Categories::Fruit => "Fruit".to_string(),
            Categories::Vegetables => "Vegetables".to_string(),
            Categories::Dairy => "Dairy".to_string(),
            Categories::Carbohydrates => "Carbohydrates".to_string(),
            Categories::Confectionary => "Confectionary".to_string(),
            Categories::Hygiene => "Hygiene".to_string(),
            Categories::Miscellaneous => "Miscellaneous".to_string(),
        }
    }

    fn string_to_enum(string: &str) -> Categories {
        match string {
            "Meat" => Categories::Meat,
            "Fish" => Categories::Fish,
            "Fruit" => Categories::Fruit,
            "Vegetables" => Categories::Vegetables,
            "Dairy" => Categories::Dairy,
            "Carbohydrates" => Categories::Carbohydrates,
            "Confectionary" => Categories::Confectionary,
            "Hygiene" => Categories::Hygiene,
            _ => Categories::Miscellaneous,
        }
    }
}

struct Item {
    id: i32,
    name: String,
    category: Cell<Categories>,
}

impl Item {
    fn new(identifier: i32, item_name: String, cat: Categories) -> Self {
        Self {
            id: identifier,
            name: item_name,
            category: Cell::new(cat),
        }
    }

    fn change_category(&self, cat: Categories) {
        self.category.set(cat);
    }
}

struct List {
    id: i32,
    date: chrono::NaiveDate,
    items: Vec<Item>,
    total_cost:i32,
}

impl List {
    fn new(identifier: i32, items_vec: Vec<Item>, total: i32) -> Self{
        Self {
            id: identifier,
            date: chrono::Local::now().naive_local().into(),
            items: items_vec,
            total_cost: total,
        }
    }

    fn date_as_string(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }
}
