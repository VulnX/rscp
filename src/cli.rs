use std::path::PathBuf;

use clap::Parser;


use crate::server;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Sends a file
    Send {
        #[clap(value_parser)]
        /// Path to a file
        filepath: std::path::PathBuf,
    },
    /// Receives a file
    Recv {
        #[clap(value_parser)]
        /// Receiving directory (default : ./files)
        save_path: Option<std::path::PathBuf>,
    },
}

pub async fn start() {
    let args = Args::parse();
    let action;
    match args.command {
        Command::Send { mut filepath } => match std::fs::metadata(&filepath) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    eprintln!("Found directory, please select a file instead.");
                    std::process::exit(1);
                }
                filepath = filepath.canonicalize().unwrap();
                action = server::Action::Download { file_path: filepath };
            }
            _ => {
                eprintln!("No such file : {:?}", filepath);
                std::process::exit(1);
            }
        },
        Command::Recv { save_path } => {
            let files_dir = PathBuf::from("./files");
            let mut save_path = save_path.unwrap_or_else(|| {
                std::fs::create_dir(&files_dir).unwrap_or_else(|err| {
                    if !err.kind().eq(&std::io::ErrorKind::AlreadyExists) {
                        panic!("Failed to create ./files directory : {:?}", err)
                    }
                });
                files_dir.clone()
            });
            if !save_path.is_dir() {
                eprintln!("No such directory : {:?}. Reverting to default.", save_path);
                std::fs::create_dir(&files_dir).unwrap_or_else(|err| {
                    if !err.kind().eq(&std::io::ErrorKind::AlreadyExists) {
                        panic!("Failed to create ./files directory : {:?}", err)
                    }
                });
                save_path = files_dir;
            }
            save_path = save_path.canonicalize().unwrap();
            action = server::Action::Upload { recv_path: save_path };
        }
    }
    let _ = server::start_server(action).await;
}
