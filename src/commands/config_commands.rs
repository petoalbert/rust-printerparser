use crate::db_ops::{SqliteDB, DB};

pub fn run_set_name_command(db_path: String, value: String) {
    let conn = SqliteDB::open(&db_path).expect("Cannot open DB");
    conn.write_config("name", &value)
        .expect("Couldn't write name")
}

pub fn run_get_name_command(db_path: String) {
    let conn = SqliteDB::open(&db_path).expect("Cannot open DB");
    let name = conn
        .read_config("name")
        .expect("Cannot read name")
        .expect("Name is None");
    println!("{}", name)
}
