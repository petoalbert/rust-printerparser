use parserprinter::{
    cli::{parse_args, Commands},
    commands::{
        commit_command::run_commit_command,
        config_commands::{run_get_name_command, run_set_name_command},
        init_command::run_init_command,
        list_branches_command::run_list_branches,
        log_checkpoints_command::run_log_checkpoints_command,
        new_branch_command::run_new_branch_command,
        restore_command::run_restore_command,
        switch_command::run_switch_command,
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
        Commands::Restore {
            db_path,
            file_path,
            hash,
        } => run_restore_command(&file_path, &db_path, &hash),
        Commands::NewBranch {
            db_path,
            branch_name,
        } => run_new_branch_command(&db_path, branch_name),
        Commands::ListBranches { db_path } => run_list_branches(&db_path),
        Commands::Switch {
            db_path,
            branch,
            file_path,
        } => run_switch_command(&db_path, &branch, &file_path),
        Commands::LogCheckpoints { db_path, branch } => {
            run_log_checkpoints_command(&db_path, branch)
        }
        Commands::Init { db_path } => run_init_command(&db_path),
    }
}
