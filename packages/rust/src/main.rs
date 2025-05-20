mod commands;

use std::env;
use std::process;

use commands::new::NewConfig;
use commands::compile::CompileConfig;

const PROGRAM_NAME: &str = "Partisia";

#[derive(Debug)]
enum Command {
    New(String, Option<String>),  // dapp_name, output_dir
    Compile,
    Deploy,
}

fn main() {
    let mut args = env::args();
    // Skip program name
    args.next();

    let command = match args.next() {
        Some(cmd) => match cmd.as_str() {
            "new" => {
                let dapp_name = match args.next() {
                    Some(name) => name,
                    None => {
                        show_usage("Dapp name is required for 'new' command");
                        process::exit(1);
                    }
                };
                let output_dir = args.next();
                Command::New(dapp_name, output_dir)
            },
            "compile" => Command::Compile,
            "deploy" => Command::Deploy,
            _ => {
                show_usage(&format!("Unknown command: {}", cmd));
                process::exit(1);
            }
        },
        None => {
            show_usage("Command is required");
            process::exit(1);
        }
    };

    let result = match command {
        Command::New(dapp_name, output_dir) => {
            let config = NewConfig::new(dapp_name, output_dir);
            commands::new::execute(config)
        },
        Command::Compile => {
            let config = CompileConfig::new();
            commands::compile::execute(config)
        },
        Command::Deploy => {
            eprintln!("Deploy command not yet implemented");
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

fn show_usage(error: &str) {
    eprintln!("❌ Error: {}", error);
    eprintln!("\nUsage: {} <command> [arguments]", PROGRAM_NAME);
    eprintln!("\nCommands:");
    eprintln!("  new <dapp-name> [output-dir]    Create a new Partisia dapp");
    eprintln!("  compile                         Compile all contracts in the contracts directory");
    eprintln!("  deploy                         Deploy a contract (not yet implemented)");
    eprintln!("\nExamples:");
    eprintln!("  {} new my-dapp", PROGRAM_NAME);
    eprintln!("  {} compile", PROGRAM_NAME);
}
