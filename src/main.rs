use clap::{Parser, Subcommand};
use std::fs;

use mkd_reader::ast::{lexer::*, token::*};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Format a markdown file to a readable file
    Format {
        path: String,
    },
    /// Format an inline markdown content to a readable content
    Inline {
        content: String,
    }
}


fn main() {
    let cli_args = Cli::parse();

    match cli_args.command {
        Some(Commands::Format { path }) => {
            format_file(path);
        },
        Some(Commands::Inline { content }) => {
            repl(content);
        }
        _ => {}
    }
}

fn format_file(file_path: String) {
    let file = fs::read_to_string(file_path);
    match file {
        Ok(content) => {
            repl(content);
        },
        Err(e) => panic!("{e}")
    }
}

fn repl(content: String) {
    let mut l: Lexer = Lexer::new(content);
    let mut token: Token = l.next_token();
    while token.token_type != TokenType::EOF && token.token_type != TokenType::ILLEGAL {
        println!("{:?}", token);
        token = l.next_token();
    }
    println!("{:?}", token);
}
