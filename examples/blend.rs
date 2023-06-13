use parserprinter::{
    cli::{parse_args, Commands},
    commands::{
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
    }
}
