pub mod data_structures;

use sqlite::State;
use data_structures::*;

use std::path::Path;

fn get_db_path() -> String {
    let bpath: String = match  std::env::current_exe() {
        Ok(exe_path) => exe_path.display().to_string(),
        Err(_) => String::new(),
    };
    let db_path = bpath.trim_end_matches("shoppinglistanalyzer").to_string() + "database.db";
    db_path
}

pub fn start_database() {
    let db_path = get_db_path();
    let existing_db = Path::new(&db_path).exists();

    let connection = sqlite::open(db_path).unwrap();

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
            FOREIGN KEY (ItemId) REFERENCES items(ItemId)
        );
        ";
        connection.execute(query).unwrap();
    }
}

pub fn store_list(list: &List) {
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = format!("
        INSERT INTO lists (Date, TotalCost) VALUES (\"{}\", {});
        ", list.date, list.get_total_cost());
        connection.execute(query).unwrap();

        let list_id_query = "
            SELECT ListId FROM lists
            ORDER BY ListId DESC
            LIMIT 1;
            ";
        let mut list_id_statement = connection.prepare(list_id_query).expect("Failed to prepare find list id statement");
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
            insert_item(item.clone());

            let item_id_query = "
                SELECT ItemId FROM items
                WHERE name = ?1
                LIMIT 1;
                ";
                let mut item_id_statement = connection.prepare(item_id_query).expect("Failed to prepare statement");
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
                    connection.execute(list_item_pair_query).unwrap();
        }
}

pub fn insert_item(item: Item) {
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = format!("
        INSERT INTO items (Name, Category) VALUES (\"{}\", \"{}\");
        ", item.name, item.category);
    if !check_item_exists(item) {
        connection.execute(query).unwrap();
    }
}

fn check_item_exists(item: Item) -> bool {
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = format!("
        SELECT EXISTS(SELECT Name FROM items WHERE Name = \"{}\");
        ", item.name);
    let mut statement = connection.prepare(query).expect("failed to prepare statement");
    if let State::Row = statement.next().expect("Failed to read") {
        let result: i64 = statement.read(0).expect("Failed to read");
        if result == 1 {
            println!("{} exists", item.name);
            return true
        }
        else {
            println!("{} does not exist", item.name);
            return false
        }
    }
    false
}

pub fn get_lists_dates() -> Vec<String> {
    let mut list = Vec::new();
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = "
        SELECT ListId, Date FROM lists
        ";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<String, _>("ListId").unwrap();
        let date = statement.read::<String, _>("Date").unwrap();
        list.push(format!("{} - {}", id, date));
    }
    list
}

pub fn get_items(list_id: i64) -> Vec<String>{
    let mut list = Vec::new();
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = "
        SELECT i.*
        FROM items i
        INNER JOIN listItems li ON i.ItemId = li.ItemId
        WHERE li.ListId = ?1;
        ";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, list_id)).expect("failed to bind list ID in get items");
    while let Ok(State::Row) = statement.next() {
        list.push(statement.read::<String, _>("Name").unwrap());
    }
    list
}
