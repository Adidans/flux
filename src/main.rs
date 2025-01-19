pub mod lexer;

use lexer::Lexer;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use ariadne::{Label, Report, ReportKind, Source};

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Tokenize {
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Tokenize { file }) => {
            let file_name = file.file_name();
            let mut file = match File::open(file) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    exit(1);
                }
            };
            let mut file_contents = String::new();
            match file.read_to_string(&mut file_contents) {
                Ok(_) => {
                    let mut lexer = Lexer::new(file_contents);
                    let tokens = lexer.tokenize();
                    for token in &tokens {
                        println!("{:?}", token);
                    }
                    for _error in &lexer.errors {
                        let _ = Report::build(ReportKind::Error, ("example.flux", 12..12))
                            .with_message("Unexpected character")
                            .finish()
                            .eprint((
                                "example.flux",
                                Source::from(include_str!("../example.flux")),
                            ));
                    }
                    if !&lexer.errors.is_empty() {
                        exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    exit(1);
                }
            }
        }
        None => {
            println!("Starting REPL");
        }
    }
}
