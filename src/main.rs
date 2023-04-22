use clap::{Parser, Subcommand};

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
        Some(Commands::Format {  path }) => {
            println!("Here is the path: {}", path);
        },
        Some(Commands::Inline { content }) => {
            inline(content);
        }
        _ => {}
    }
}

fn format_file(path: String) {
    todo!();
}

fn inline(content: String) {
    let mut l: Lexer = Lexer::new(content);
    let mut token: Token = l.next_token();
    while token.token_type != TokenType::EOF && token.token_type != TokenType::ILLEGAL {
        println!("{:?}", token);
        token = l.next_token();
    }
    println!("{:?}", token);
}
