use parserprinter::{
    cli::{parse_args, Commands},
    commands::{
        checkout_command::run_checkout_command,
        commit_command::run_commit_command,
        config_commands::{run_get_name_command, run_set_name_command},
        test_command::run_command_test,
    },
};

fn main() {
    let args = parse_args();
    match args.command {
        Commands::Test { from_path, to_path } => run_command_test(from_path, to_path),
        Commands::SetName { value } => run_set_name_command(value),
        Commands::GetName => run_get_name_command(),
        Commands::Commit { file_path } => run_commit_command(&file_path, "./test.sqlite"),
        Commands::Checkout { file_path, hash } => {
            run_checkout_command(&file_path, "./test.sqlite", &hash)
        }
    }
}
