#[derive(Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: f32,
}

impl Item {
    pub fn new(identifier: i32, item_name: String, cat: String, m_price: f32) -> Self {
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

    pub fn get_total_cost(&self) -> f32 {
        let mut total: f32 = 0.0;
        for item in self.items.iter() {
            total += item.price;
        }
        total
    }
}
