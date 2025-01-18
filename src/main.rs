mod lexer {
    pub mod lexer;
    pub mod token;
}

use lexer::token::{Token, TokenType};

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

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
                    println!("{}", file_contents);
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    exit(1);
                }
            }
        }
        None => {
            // println!("Starting REPL");
            let token = Token::new(TokenType::Integer(65), 1);
            match token.token_type {
                TokenType::Integer(i) => {
                    println!("Integer: {} Line: {}", i, token.line);
                }
                _ => {
                    println!("Not an integer");
                }
            }
            println!("{:?}", token);
        }
    }
}
