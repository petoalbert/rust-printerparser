use parserprinter::{
    cli::{parse_args, Commands},
    commands::{
        checkout_command::run_checkout_command,
        commit_command::run_commit_command,
        config_commands::{run_get_name_command, run_set_name_command},
        list_branches_command::run_list_branches,
        log_command::run_log_command,
        new_branch_command::run_new_branch_command,
        test_command::run_command_test,
    },
};

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
        } => run_commit_command(&file_path, &db_path, message),
        Commands::Checkout {
            db_path,
            file_path,
            hash,
        } => run_checkout_command(&file_path, &db_path, &hash),
        Commands::Log { db_path } => run_log_command(&db_path),
        Commands::NewBranch {
            db_path,
            branch_name,
        } => run_new_branch_command(&db_path, branch_name),
        Commands::ListBranches { db_path } => run_list_branches(&db_path),
    }
}
