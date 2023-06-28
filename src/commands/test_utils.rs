use super::init_command::run_init_command;

pub fn init_db(db_path: &str) {
    run_init_command(db_path)
}