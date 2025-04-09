use sqlite::State;
use crate::data_structures::*;
use std::{cell::RefCell, path::Path, rc::Rc};
use gtk4::glib::DateTime;

pub struct Database {
    path: String,
    connection: Option<sqlite::Connection>,
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
                println!("Found list ID: {}", list_id);
                drop(list_id_statement);
            } else {
                println!("List id not found.");
            }

            for item in list.items.iter() {
                item.print_item();
                self.insert_item(item);

                let item_id_query = "
                    SELECT ItemId FROM items
                    WHERE name = ?1
                    LIMIT 1;
                ";
                let mut item_id_statement = self.connection.as_ref().unwrap().prepare(item_id_query).expect("Failed to prepare statement");
                let item_name: &str = &item.name;
                item_id_statement.bind((1, item_name)).expect("Failed to bind item id");

                let mut item_id: i64 = 0;
                if let State::Row = item_id_statement.next().expect("Failed to execute find item ID query") {
                    item_id = item_id_statement.read(0).expect("Failed to read ID");
                } else {
                    println!("Item not found.");
                }

                let list_item_pair_query = format!("
                    INSERT INTO listItems (ListId, ItemId, Price) VALUES ({}, {}, {});
                    ", list_id, item_id, item.price);
                    self.connection.as_ref().unwrap().execute(list_item_pair_query).unwrap();
            }
    }

    pub fn insert_item(&self, item: &Item) {
        let item_rc = Rc::new(RefCell::new(item));
        let query = format!("
            INSERT INTO items (Name, Category) VALUES (\"{}\", \"{}\");
            ", item.name, item.category);
            if !self.check_item_exists(Rc::clone(&item_rc)) {
                self.connection.as_ref().unwrap().execute(query).expect("query failed for inserting item");
                println!("query has executed correctly");
            }
    }

    fn check_item_exists(&self, item: Rc<RefCell<&Item>>) -> bool {
        let query = format!("
            SELECT EXISTS(SELECT Name FROM items WHERE Name = \"{}\");
            ", item.borrow_mut().name);
            let mut statement = self.connection.as_ref().unwrap().prepare(query).expect("failed to prepare statement");
            if let State::Row = statement.next().expect("Failed to read") {
                let result: i64 = statement.read(0).expect("Failed to read");
                if result == 1 {
                    println!("{} exists", item.borrow_mut().name);
                    return true
                }
                else {
                    println!("{} does not exist", item.borrow_mut().name);
                    return false
                }
            }
            false
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
        let start_string: &str = &format!("{}-{}-{}", start_date.day_of_month(), start_date.month(), start_date.year());
        let end_string: &str = &format!("{}-{}-{}", end_date.day_of_month(), end_date.month(), end_date.year());
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
            WHERE li.ItemId = ?1;
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
}
