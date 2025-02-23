use sqlite;

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

        CREATE TABLE items (ItemId INTEGER PRIMARY KEY AUTOINCREMENT, Name TEXT);

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

pub fn insert_item(name: String) {
    let db_path = get_db_path();
    let connection = sqlite::open(db_path).unwrap();
    let query = format!("
        INSERT INTO items (Name) VALUES (\"{}\");
        ", name);
    connection.execute(query).unwrap();
}
