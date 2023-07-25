use crate::db::db_ops::{Persistence, DB};

pub fn run_set_name_command(db_path: String, value: String) {
    let mut conn = Persistence::open(&db_path).expect("Cannot open DB");
    conn.execute_in_transaction(|tx| Persistence::write_config(tx, "name", &value))
        .expect("Couldn't write name")
}

pub fn run_get_name_command(db_path: String) {
    let conn = Persistence::open(&db_path).expect("Cannot open DB");
    let name = conn
        .read_config("name")
        .expect("Cannot read name")
        .expect("Name is None");
    println!("{}", name)
}
