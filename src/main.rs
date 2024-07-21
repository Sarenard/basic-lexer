#![feature(fs_try_exists)]

use clap::Parser;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod compiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the file to open
    #[arg(short, long)]
    file: String,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args = Args::parse();

    let file = lines_from_file(args.file);

    let tokens = compiler::lexer::lex(file);

    println!("{:?}", tokens);
}
