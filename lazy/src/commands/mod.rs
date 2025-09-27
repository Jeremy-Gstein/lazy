pub mod rm;
pub mod run;
pub mod build;

use crate::cli::Commands;
use crate::config::LazyConfig;

pub fn dispatch_command(cmd: Commands, config: &LazyConfig) {
    match cmd {
        Commands::Rm => rm::run(),
        Commands::Run => run::run(),
        Commands::Build => build::run(config),
    }
}

// pub fn run_command_chain(seq: &[String]) {
//     for cmd in seq {
//         match cmd.as_str() {
//             "rm" => rm::run(),
//             "run" => run::run(),
//             "build" => build::run(),
//             _ => println!("Unknown command: {}", cmd),
//         }
//     }
// }

