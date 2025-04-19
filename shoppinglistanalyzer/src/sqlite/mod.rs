use sqlite::State;
use crate::data_structures::*;
use std::{cell::RefCell, path::Path, rc::Rc};
use gtk4::glib::DateTime;

pub struct Database {
    path: String,
    pub connection: Option<sqlite::Connection>,
}

impl Database {
    pub fn new() -> Self {
        let db_path = Self::get_db_path();
        Self {
            path: db_path.to_string(),
            connection: None
        }
    }
    fn get_db_path() -> String {
        let bpath: String = match  std::env::current_exe() {
            Ok(exe_path) => exe_path.display().to_string(),
            Err(_) => String::new(),
        };
        let db_path = bpath.trim_end_matches("shoppinglistanalyzer").to_string() + "database.db";
        db_path
    }

    pub fn start_database(&mut self) {
        let existing_db = Path::new(&self.path).exists();
        self.connection = Some(sqlite::open(self.path.clone()).unwrap());

        if !existing_db {
            let query = "
                CREATE TABLE lists (ListId INTEGER PRIMARY KEY AUTOINCREMENT, Date TEXT, TotalCost REAL);

            CREATE TABLE items (ItemId INTEGER PRIMARY KEY AUTOINCREMENT, Name TEXT UNIQUE, Category TEXT);

            CREATE TABLE listItems (ListId INTEGER, 
                ItemId INTEGER,
                Price REAL,
                FOREIGN KEY (ListId) REFERENCES lists(ListId),
                FOREIGN KEY (ItemId) REFERENCES items(ItemId));

            CREATE TABLE nutrition (ItemId INTEGER PRIMARY KEY, 
                Nutriscore TEXT,
                Energy INTEGER,
                Fat INTEGER,
                SaturatedFats INTEGER,
                Carbohydrates INTEGER,
                Sugar INTEGER,
                Fiber INTEGER,
                Proteins INTEGER,
                Salt INTEGER,
                ImgURL TEXT,
                FOREIGN KEY (ItemId) REFERENCES items(ItemId)
            );
            ";
            self.connection.as_ref().unwrap().execute(query).unwrap();
        }
    }

    pub fn store_list(&self, list: &List) {
        let query = format!("
            INSERT INTO lists (Date, TotalCost) VALUES (\"{}\", {});
            ", list.date, list.get_total_cost());
            self.connection.as_ref().unwrap().execute(query).unwrap();

            let list_id_query = "
                SELECT ListId FROM lists
                ORDER BY ListId DESC
                LIMIT 1;
            ";
            let mut list_id_statement = self.connection.as_ref().unwrap().prepare(list_id_query).expect("Failed to prepare find list id statement");
            let mut list_id: i64 = 0;
            if let State::Row = list_id_statement.next().expect("Failed to execute find item ID query") {
                list_id = list_id_statement.read(0).expect("Failed to read ID");
                drop(list_id_statement);
            } else {
                println!("List id not found.");
            }

            for item in list.items.iter() {
                self.insert_item(item);

                let item_id: i64 = self.get_item_id(item.name.clone());

                let list_item_pair_query = format!("
                    INSERT INTO listItems (ListId, ItemId, Price) VALUES ({}, {}, {});
                    ", list_id, item_id, item.price);
                    self.connection.as_ref().unwrap().execute(list_item_pair_query).unwrap();
            }
    }

    pub fn update_list(&self, list: &List) {
        // Update list date and total cost
        let mut query = format!("
            UPDATE lists
            SET Date = \"{}\",
                TotalCost = \"{}\"
            WHERE ListId = {};
            ", list.date, list.get_total_cost(), list.id);

        let mut items:Vec<Item> = Vec::new();

        // Add or update items
        for item in list.items.iter() {
            let item_rc = Rc::new(RefCell::new(item));
            if !self.check_item_exists(Rc::clone(&item_rc)) {
                let new_item_query = format!("
                    INSERT INTO items (Name, Category) VALUES (\"{}\", \"{}\");
                    ", item.name, item.category);
                self.connection.as_ref().unwrap().execute(new_item_query).unwrap();
                items.push(Item::new(self.get_item_id(item.name.clone()), item.name.clone(), item.category.clone(), item.price));
            } else {
                query.push_str(&format!("
                        UPDATE items
                        SET Name = \"{}\",
                            Category = \"{}\"
                        WHERE ItemId = {};
                    ", item.name, item.category, item.id));
                items.push(Item::new(self.get_item_id(item.name.clone()), item.name.clone(), item.category.clone(), item.price));
            }
        }
        self.connection.as_ref().unwrap().execute(query).unwrap();

        let clear_references_query = format!("
            DELETE FROM listItems
            WHERE ListId = {};
            ", list.id);
        self.connection.as_ref().unwrap().execute(clear_references_query).unwrap();

        let mut references_query = String::new();
        for item in items.iter() {
            references_query.push_str(&format!("
                    INSERT INTO listItems (ListId, ItemId, Price) VALUES (\"{}\", \"{}\", \"{}\");
                    ", list.id, item.id, item.price));
        }
        self.connection.as_ref().unwrap().execute(references_query).unwrap();
    }

    pub fn delete_list(&self, list_id: i64) {
        let query = format!("
            DELETE FROM listItems
            WHERE ListId = {};

            DELETE FROM lists
            WHERE ListId = {};
            ", list_id, list_id);
        self.connection.as_ref().unwrap().execute(query).unwrap();
    }

    pub fn insert_item(&self, item: &Item) {
        let item_rc = Rc::new(RefCell::new(item));
        if !self.check_item_exists(Rc::clone(&item_rc)) {
            let query = format!("
                INSERT INTO items (Name, Category) VALUES (\"{}\", \"{}\");
                ", item.name, item.category);
                self.connection.as_ref().unwrap().execute(query).expect("query failed for inserting item");
        } 
    }

    fn check_item_exists(&self, item: Rc<RefCell<&Item>>) -> bool {
        let query = format!("
            SELECT EXISTS(SELECT Name FROM items WHERE Name = \"{}\");
            ", item.borrow_mut().name);
            let mut statement = self.connection.as_ref().unwrap().prepare(query).expect("failed to prepare statement");
            if let State::Row = statement.next().expect("Failed to read") {
                let result: i64 = statement.read(0).expect("Failed to read");
                return result == 1;
            }
            false
    }

    pub fn get_list(&self, list_id: i64) -> List {
        let query = format!("
            SELECT * FROM lists
            WHERE ListId = {}
            LIMIT 1;
            ", list_id);
            let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
            let mut date: String = "".to_string();
            let items = self.get_items_by_list_id(list_id);
            while let Ok(State::Row) = statement.next() {
                date = statement.read::<String, _>("Date").unwrap();
            }
            List::new(list_id, items, date)
    }

    pub fn get_lists_dates(&self) -> Vec<String> {
        let mut list = Vec::new();
        let query = "
            SELECT ListId, Date FROM lists
            ";
        let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let id = statement.read::<String, _>("ListId").unwrap();
            let date = statement.read::<String, _>("Date").unwrap();
            list.push(format!("{} - {}", id, date));
        }
        list
    }

    pub fn get_lists_in_dates_range(&self, start_date: &DateTime, end_date: &DateTime) -> Vec<List> {
        let mut lists: Vec<List> = Vec::new();
        let start_string: &str = &format!("{:04}-{:02}-{:02}", start_date.year(), start_date.month(), start_date.day_of_month());
        let end_string: &str = &format!("{:04}-{:02}-{:02}", end_date.year(), end_date.month(), end_date.day_of_month());
        let query = format!("
            SELECT * 
            FROM lists
            WHERE Date BETWEEN '{}' AND '{}';
            ", start_string, end_string);
            let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
            while let Ok(State::Row) = statement.next() {
                let id = statement.read::<i64, _>("ListId").unwrap();
                let date = statement.read::<String, _>("Date").unwrap(); 
                let items = self.get_items_by_list_id(id);
                lists.push(List::new(id, items, date));
            }
            lists
    }

    pub fn get_items(&self) -> Vec<Item> {
        let mut list = Vec::new();
        let query = "
            SELECT * FROM items
            ORDER BY Name;
            ";
        let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let id = statement.read::<i64, _>("ItemId").unwrap();
            let name = statement.read::<String, _>("Name").unwrap();
            let category = statement.read::<String, _>("Category").unwrap();
            list.push(Item::new(id, name, category, 0.0));
        }
        list
    }

    pub fn get_items_in_lists(&self) -> Vec<Item> {
        let mut list = Vec::new();
        let query = "
            SELECT DISTINCT i.*
            FROM items i
            INNER JOIN listItems l ON i.itemId = l.itemId
            ORDER BY Name;
            ";
        let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let id = statement.read::<i64, _>("ItemId").unwrap();
            let name = statement.read::<String, _>("Name").unwrap();
            let category = statement.read::<String, _>("Category").unwrap();
            list.push(Item::new(id, name, category, 0.0));
        }
        list
    }

    pub fn get_items_by_item_id(&self, item_id: i64) -> Vec<ListItem> {
        let mut list = Vec::new();
        let query = "
            SELECT i.*, l.Date, li.price, li.ListId
            FROM listItems li
            INNER JOIN items i ON i.ItemId = li.ItemId
            INNER JOIN lists l ON l.ListId = li.ListId
            WHERE li.ItemId = ?1
            ORDER BY l.Date;
        ";
        let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
        statement.bind((1, item_id)).expect("failed to bind list ID in get items");
        while let Ok(State::Row) = statement.next() {
            let item_id = statement.read::<i64, _>("ItemId").unwrap();
            let list_id = statement.read::<i64, _>("ListId").unwrap();
            let name = statement.read::<String, _>("Name").unwrap();
            let category = statement.read::<String, _>("Category").unwrap();
            let date = statement.read::<String, _>("Date").unwrap();
            let price = statement.read::<f64, _>("Price").unwrap();
            list.push(ListItem::new(item_id, list_id, name, category, price, date));
        }
        list
    }

    pub fn get_items_by_list_id(&self, list_id: i64) -> Vec<Item> {
        let mut list = Vec::new();
        let query = "
            SELECT i.*, li.price
            FROM items i
            INNER JOIN listItems li ON i.ItemId = li.ItemId
            WHERE li.ListId = ?1;
        ";
        let mut statement = self.connection.as_ref().unwrap().prepare(query).unwrap();
        statement.bind((1, list_id)).expect("failed to bind list ID in get items");
        while let Ok(State::Row) = statement.next() {
            let id = statement.read::<i64, _>("ItemId").unwrap();
            let name = statement.read::<String, _>("Name").unwrap();
            let category = statement.read::<String, _>("Category").unwrap();
            let price = statement.read::<f64, _>("Price").unwrap();
            list.push(Item::new(id, name, category, price));
        }
        list
    }

    fn get_item_id(&self, item_name: String) -> i64 {
        let item_id_query = "
            SELECT ItemId FROM items
            WHERE name = ?1
            LIMIT 1;
        ";
        let mut item_id_statement = self.connection.as_ref().unwrap().prepare(item_id_query).expect("Failed to prepare statement");
        let item_name: &str = &item_name;
        item_id_statement.bind((1, item_name)).expect("Failed to bind item id");

        if let State::Row = item_id_statement.next().expect("Failed to execute find item ID query") {
            item_id_statement.read(0).expect("Failed to read ID")
        } else {
            0
        }
    }
}
