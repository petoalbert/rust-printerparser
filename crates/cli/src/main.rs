mod cli;

use cli::{parse_args, Commands};

use parserprinter::api::{
    commit_command::create_new_commit,
    config_commands::{run_get_name_command, run_set_name_command},
    init_command::run_init_command,
    list_branches_command::list_braches,
    log_checkpoints_command::log_checkpoints,
    new_branch_command::run_new_branch,
    restore_command::restore_checkpoint,
    switch_command::switch_branches,
    test_command::run_command_test,
};

fn print_all_branches(db_path: &str) {
    let branches = list_braches(db_path);
    for branch in branches {
        println!("{}", branch)
    }
}

fn print_checkpoints(db_path: &str, branch_name: Option<String>) {
    let commits = log_checkpoints(db_path, branch_name);
    for commit in commits {
        println!("{} {} {}", commit.hash, commit.branch, commit.message)
    }
}

fn run_new_branch_command(db_path: &str, new_branch_name: &str) {
    run_new_branch(db_path, new_branch_name).expect("Cannot create new branch")
}

fn run_create_new_commit(db_path: &str, file_path: &str, message: Option<String>) {
    create_new_commit(file_path, db_path, message).expect("Cannot create new commit")
}

fn run_restore_checkpoint(db_path: &str, file_path: &str, hash: &str) {
    restore_checkpoint(file_path, db_path, hash).expect("Cannot restore checkpoint")
}

fn run_switch_branches(db_path: &str, file_path: &str, branch_name: &str) {
    switch_branches(db_path, branch_name, file_path).expect("Cannot switch branches")
}

fn main() {
    let args = parse_args();
    match args.command {
        Commands::Test { from_path, to_path } => run_command_test(from_path, to_path),
        Commands::SetName { value, db_path } => run_set_name_command(db_path, value),
        Commands::GetName { db_path } => run_get_name_command(db_path),
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
        Commands::Switch {
            db_path,
            branch,
            file_path,
        } => run_switch_branches(&db_path, &branch, &file_path),
        Commands::LogCheckpoints { db_path, branch } => print_checkpoints(&db_path, branch),
        Commands::Init { db_path } => run_init_command(&db_path),
    }
}
