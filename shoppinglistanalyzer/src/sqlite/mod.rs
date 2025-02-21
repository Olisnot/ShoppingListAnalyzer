use sqlite;

pub fn create_database() {
    let bpath: String;
    match  std::env::current_exe() {
        Ok(exe_path) => bpath = exe_path.display().to_string(),
        Err(_) => bpath = String::new(),
    };

    let dbpath = bpath.trim_end_matches("shoppinglistanalyzer").to_string() + "database.db";

    let connection = sqlite::open(dbpath).unwrap();

    let query = "
        CREATE TABLE lists (ListId INTEGER, Date INTEGER, TotalCost REAL);
        CREATE TABLE items (ItemId INTEGER, Name TEXT);
        CREATE TABLE listItems (ListId INTEGER, ItemId INTEGER, Price INTEGER);
        CREATE TABLE nutrition (ItemId INTEGER, 
                                Nutriscore TEXT,
                                Energy INTEGER,
                                Fat INTEGER,
                                SaturatedFats INTEGER,
                                Carbohydrates INTEGER,
                                Sugar INTEGER,
                                Fiber INTEGER,
                                Proteins INTEGER,
                                Salt INTEGER);
    ";
    connection.execute(query).unwrap();
}
