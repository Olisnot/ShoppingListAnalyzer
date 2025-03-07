use chrono;

#[derive(Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: i32,
}

impl Item {
    fn new(identifier: i32, item_name: String, cat: String, m_price: i32) -> Self {
        Self {
            id: identifier,
            name: item_name,
            category: cat,
            price: m_price,
        }
    }

    fn change_category(&mut self, cat: String) {
        self.category = cat;
    }
}

pub struct List {
    pub id: i32,
    pub date: chrono::NaiveDate,
    pub items: Vec<Item>,
    pub total_cost:i32,
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
