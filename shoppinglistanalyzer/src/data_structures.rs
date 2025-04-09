pub enum Categories {
    Protein,
    FruitsVegetables,
    Dairy,
    Carbohydrates,
    FatsOils,
    Unhealthy,
    Hygiene,
    Misc,
}

impl Categories {
    pub fn to_cat_string(&self) -> String {
        match self {
            Categories::Protein => "Protein".to_string(),
            Categories::FruitsVegetables => "Fruit/Vegetable".to_string(),
            Categories::Dairy => "Dairy".to_string(),
            Categories::Carbohydrates => "Carbohydrate".to_string(),
            Categories::FatsOils => "Fat/Oil".to_string(),
            Categories::Unhealthy => "Unhealthy".to_string(),
            Categories::Hygiene => "Hygiene".to_string(),
            Categories::Misc => "Miscellaneous".to_string()
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub price: f64,
}

impl Item {
    pub fn new(identifier: i64, item_name: String, cat: String, m_price: f64) -> Self {
        Self {
            id: identifier,
            name: item_name,
            category: cat,
            price: m_price,
        }
    }

    pub fn print_item(&self) {
        println!("---------------------------------------");
        print!("id: {},\nname: {}\ncat: {}\nprice: {}\n", self.id, self.name, self.category, self.price);
        println!("---------------------------------------");
    }

    pub fn change_category(&mut self, cat: String) {
        self.category = cat;
    }
}

pub struct List {
    pub id: i64,
    pub date: String,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(identifier: i64, items_vec: Vec<Item>, date_of_list: String) -> Self{
        Self {
            id: identifier,
            date: date_of_list,
            items: items_vec,
        }
    }

    pub fn get_total_cost(&self) -> f64 {
        let mut total: f64 = 0.0;
        for item in self.items.iter() {
            total += item.price;
        }
        total
    }
}

pub struct ListItem {
    pub item_id: i64,
    pub list_id: i64,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub date: String,
}
impl ListItem {
    pub fn new(item_id: i64, list_id: i64, name: String, category: String, price: f64, date: String) -> Self {
        Self {
            item_id,
            list_id, 
            name,
            category,
            price,
            date,
        }
    }

    pub fn change_category(&mut self, cat: String) {
        self.category = cat;
    }
}
