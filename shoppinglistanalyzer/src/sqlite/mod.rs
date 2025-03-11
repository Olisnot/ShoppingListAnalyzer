pub mod data_structures;

use sqlite;
use sqlite::State;
use data_structures::*;

use std::path::Path;

fn get_db_path() -> String {
    let bpath: String;
    match  std::env::current_exe() {
        Ok(exe_path) => bpath = exe_path.display().to_string(),
        Err(_) => bpath = String::new(),
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
        CREATE TABLE lists (ListId INTEGER PRIMARY KEY AUTOINCREMENT, Date INTEGER, TotalCost REAL);

        CREATE TABLE items (ItemId INTEGER PRIMARY KEY AUTOINCREMENT, Name TEXT UNIQUE, Category TEXT, Price REAL);

        CREATE TABLE listItems (ListId INTEGER, 
            ItemId INTEGER,
            Price INTEGER,
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
        INSERT INTO lists (Date, TotalCost) VALUES ({}, {});
        ", list.date, list.get_total_cost());
        connection.execute(query).unwrap();

    let list_id_query = format!("
        SELECT ListId FROM lists
        WHERE date = {} AND TotalCost = {}
        LIMIT 1;
        ", list.date, list.get_total_cost());
    println!("{}", list_id_query);
    let list_id = connection.prepare(list_id_query).unwrap().read::<i64, _>("ListId").unwrap();
    println!("listID: {}", list_id);

        for item in list.items.iter() {
            item.print_item();
            insert_item(item.clone());

            let item_id_query = format!("
                SELECT ItemId FROM items
                WHERE name = \"{}\"
                LIMIT 1;
                ", item.name);
            println!("{}", item_id_query);
            let mut item_id: i64 = 0;
            let mut item_id_statement = connection.prepare(item_id_query).unwrap();
            item_id_statement.bind((0, 1)).unwrap();
            while let Ok(State::Row) = item_id_statement.next() {
                item_id = item_id_statement.read::<i64, _>("ItemId").unwrap();
                println!("item_id = {}", item_id);
            }

            let list_item_pair_query = format!("
                INSERT INTO listItems (ListId, ItemId, Price) VALUES ({}, {}, {});
                ", list_id, item_id, item.price);
            connection.execute(list_item_pair_query).unwrap();
        }
}

pub fn insert_item(item: data_structures::Item) {
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = format!("
        INSERT INTO items (Name, Category, Price) VALUES (\"{}\", \"{}\", {});
        ", item.name, item.category, item.price);
    println!("\n---------------\n{}\n----------------", query);
    connection.execute(query).unwrap();
}

pub fn get_items() -> Vec<String>{
    let mut list = Vec::new();
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = "
        SELECT name FROM items
        ";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        list.push(statement.read::<String, _>("Name").unwrap());
    }
    list
}
