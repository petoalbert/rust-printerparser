mod cli;

use cli::{parse_args, Commands};

use log::error;
use parserprinter::{
    api::{
        commit_command::create_new_commit, get_current_branch::get_current_branch,
        init_command::init_db, list_branches_command::list_braches,
        log_checkpoints_command::list_checkpoints, new_branch_command::create_new_branch,
        restore_command::restore_checkpoint, switch_command::switch_branches,
        test_command::run_command_test,
    },
    db::db_ops::DBError,
};

fn print_error_discard_rest<T>(res: Result<T, DBError>) {
    match res {
        Ok(_) => {}
        Err(err) => error!("{}", err),
    }
}

fn print_all_branches(db_path: &str) {
    let result = list_braches(db_path);
    match result {
        Ok(branches) => branches
            .into_iter()
            .for_each(|branch| println!("{}", branch)),
        Err(err) => error!("{}", err),
    }
}

fn print_checkpoints(db_path: &str, branch_name: &str) {
    let result = list_checkpoints(db_path, branch_name);
    match result {
        Ok(checkpoints) => checkpoints
            .into_iter()
            .for_each(|commit| println!("{} {} {}", commit.hash, commit.branch, commit.message)),
        Err(err) => error!("{}", err),
    }
}

fn run_new_branch_command(db_path: &str, new_branch_name: &str) {
    print_error_discard_rest(create_new_branch(db_path, new_branch_name));
}

fn run_create_new_commit(db_path: &str, file_path: &str, message: Option<String>) {
    print_error_discard_rest(create_new_commit(file_path, db_path, message));
}

fn run_restore_checkpoint(db_path: &str, file_path: &str, hash: &str) {
    print_error_discard_rest(restore_checkpoint(file_path, db_path, hash));
}

fn run_switch_branches(db_path: &str, file_path: &str, branch_name: &str) {
    print_error_discard_rest(switch_branches(db_path, branch_name, file_path));
}

fn run_get_current_branch(db_path: &str) {
    let result = get_current_branch(db_path);
    match result {
        Ok(branch) => println!("Current branch: {}", branch),
        Err(err) => error!("{}", err),
    }
}

fn run_init_command(db_path: &str) {
    print_error_discard_rest(init_db(db_path));
}

fn main() {
    let args = parse_args();
    env_logger::init();
    match args.command {
        Commands::Test { from_path, to_path } => run_command_test(from_path, to_path),
        Commands::Commit {
            db_path,
            file_path,
            message,
        } => run_create_new_commit(&db_path, &file_path, message),
        Commands::Restore {
            db_path,
            file_path,
            hash,
        } => run_restore_checkpoint(&db_path, &file_path, &hash),
        Commands::NewBranch {
            db_path,
            branch_name,
        } => run_new_branch_command(&db_path, &branch_name),
        Commands::ListBranches { db_path } => print_all_branches(&db_path),
        Commands::GetCurrentBranch { db_path } => run_get_current_branch(&db_path),
        Commands::Switch {
            db_path,
            branch,
            file_path,
        } => run_switch_branches(&db_path, &file_path, &branch),
        Commands::LogCheckpoints { db_path, branch } => print_checkpoints(&db_path, &branch),
        Commands::Init { db_path } => run_init_command(&db_path),
    }
}
