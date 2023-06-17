use crate::db_ops::{open_db, read_config, write_config};

pub fn run_set_name_command(db_path: String, value: String) {
    let conn = open_db(&db_path).expect("Cannot open DB");
    write_config(&conn, &"name".to_string(), &value).expect("Couldn't write name")
}

pub fn run_get_name_command(db_path: String) {
    let conn = open_db(&db_path).expect("Cannot open DB");
    let name = read_config(&conn, "name")
        .expect("Cannot read name")
        .expect("Name is None");
    println!("{}", name)
}
