use std::env;
use std::fs;
use std::process;

use anyhow::Result;
use lol_chat_parser::parse_log;
use serde_json::to_string_pretty;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("parse") => {
            let path = args
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing file path for `parse` command"))?;
            parse_command(&path)?;
        }
        Some("help") | None => {
            print_help();
        }
        Some("credits") => {
            print_credits();
        }
        Some(other) => {
            eprintln!("Unknown command `{other}`\n");
            print_help();
        }
    }

    Ok(())
}

fn parse_command(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let parsed = parse_log(&content);
    let json = to_string_pretty(&parsed)?;
    println!("{json}");
    Ok(())
}

fn print_help() {
    println!(
        "\
lol_chat_parser – League of Legends chat log parser

USAGE:
    lol_chat_parser <command> [args]

COMMANDS:
    parse <path>    Parse a text file with LoL chat logs and print structured JSON
    help            Show this help information
    credits         Show project credits
"
    );
}

fn print_credits() {
    println!(
        "\
lol_chat_parser – League of Legends chat log parser

Made by Bohdan Tarverdiiev"
    );
}
