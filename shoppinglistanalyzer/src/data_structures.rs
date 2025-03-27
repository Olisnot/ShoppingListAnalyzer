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
    pub id: i32,
    pub date: String,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(identifier: i32, items_vec: Vec<Item>, date_of_list: String) -> Self{
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
    pub fn cost_per_category(&self) -> Vec<(f64, String)> {
        let mut proteins = 0.0;
        let mut fruit_vegtabable = 0.0;
        let mut dairy = 0.0;
        let mut fat_oil = 0.0;
        let mut carbohydrate = 0.0;
        let mut unhealthy = 0.0;
        let mut hygiene = 0.0;
        let mut misc = 0.0;

        for item in self.items.iter() {
            if item.category == Categories::Protein.to_cat_string() {
                proteins += item.price;
            } else if item.category == Categories::FruitsVegetables.to_cat_string() {
                fruit_vegtabable += item.price;
            } else if item.category == Categories::Dairy.to_cat_string() {
                dairy += item.price;
            } else if item.category == Categories::FatsOils.to_cat_string() {
                fat_oil += item.price;
            } else if item.category == Categories::Carbohydrates.to_cat_string() {
                carbohydrate += item.price;
            } else if item.category == Categories::Unhealthy.to_cat_string() {
                unhealthy += item.price;
            } else if item.category == Categories::Hygiene.to_cat_string() {
                hygiene += item.price;
            } else if item.category == Categories::Misc.to_cat_string() || !item.category.is_empty() {
                misc += item.price;
            } 
        }
        let category_totals: Vec<(f64, String)> = vec!{
            (proteins, Categories::Protein.to_cat_string()),
            (fruit_vegtabable, Categories::FruitsVegetables.to_cat_string()),
            (dairy, Categories::Dairy.to_cat_string()),
            (fat_oil, Categories::FatsOils.to_cat_string()),
            (carbohydrate, Categories::Carbohydrates.to_cat_string()),
            (unhealthy, Categories::Unhealthy.to_cat_string()),
            (hygiene, Categories::Hygiene.to_cat_string()),
            (misc, Categories::Misc.to_cat_string()),
        };
        category_totals
    }
}
