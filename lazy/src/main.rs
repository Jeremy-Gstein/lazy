mod cli;
mod commands;
mod config;
mod utils;

use clap::Parser;
use clap::CommandFactory;
use cli::{Cli, Commands};
use commands::{add, build, run, rm, dispatch_command};
use config::LazyConfig;

fn main() {
    let cli = Cli::parse();
    let config = LazyConfig::load_default().unwrap_or_default();

    if !cli.sequence.is_empty() {
        // Interpret cli.sequence as commands or commands with arguments:
        // For example: ["add", "curl", "build", "run"]
        let mut args = cli.sequence.iter();

        while let Some(cmd_name) = args.next() {
            match cmd_name.as_str() {
                "rm" => rm::run(),
                "run" => run::run(&config),
                "build" => build::run(&config),
                "add" => {
                    // Get package name argument
                    if let Some(pkg) = args.next() {
                        add::run(&config, pkg);
                    } else {
                        eprintln!("Error: 'add' command requires a package name argument");
                        std::process::exit(1);
                    }
                }
                unknown => {
                    eprintln!("Unknown command in sequence: {}", unknown);
                    std::process::exit(1);
                }
            }
        }
    } else if let Some(cmd) = cli.command {
        dispatch_command(cmd, &config);
    } else {
        // If no args passed, print help
        Cli::command().print_help().unwrap();
    }
}



// mod cli;
// mod commands;
// mod config;
// mod utils;
//
// use clap::Parser;
// use cli::Cli;
// use commands::{add, build, run, rm, dispatch_command};
// use config::LazyConfig;
//
// fn main() {
//     let cli = Cli::parse();
//     let config = LazyConfig::load_default().unwrap_or_default();
//
//     if !cli.sequence.is_empty() {
//         for cmd_name in &cli.sequence {
//             match cmd_name.as_str() {
//                 "rm" => rm::run(),
//                 "run" => run::run(&config),
//                 "build" => build::run(&config),
//                 "add" => add::run(&config),
//                 _ => todo!(),
//                 //_ => eprintln!("Unknown command {}", cmd_name),
//             }
//         }
//     } else if let Some(cmd) = cli.command {
//         dispatch_command(cmd, &config);
//     }
// }
//
//


// mod cli;
// mod commands;
//
//
// use clap::Parser;
// use cli::Cli;
// use commands::dispatch_command;
// use commands::{build, run, rm};
//
//
// fn main() {
//     let cli = Cli::parse();
//
//     if !cli.sequence.is_empty() {
//         // sequence: rm build
//         for cmd_name in &cli.sequence {
//             match cmd_name.as_str() {
//                 "rm" => rm::run(),
//                 "build" => build::run(),
//                 "run" => run::run(),
//                 _ => eprintln!("Unknown command {}", cmd_name),
//             }
//         }
//     } else if let Some(cmd) = cli.command {
//         dispatch_command(cmd);
//     }
// }
//
// use clap::{Parser, Subcommand};
//
// #[derive(Parser, Debug)]
// #[command(name = "foo", version = "1.0", about = "A CLI utility for Docker")]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }
//
// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// Run a Docker container
//     Run {
//         #[arg(short, long)]
//         image: String,
//     },
//     /// Stop a Docker container
//     Stop {
//         #[arg(short, long)]
//         container: String,
//     },
// }
//
// fn main() {
//     let cli = Cli::parse();
//
//     match &cli.command {
//         Commands::Run { image } => {
//             println!("Running docker container for image '{}'", image);
//             // Run `docker run {image}`
//         }
//         Commands::Stop { container } => {
//             println!("Stopping docker container '{}'", container);
//             // Run `docker stop {container}`
//         }
//     }
// }
