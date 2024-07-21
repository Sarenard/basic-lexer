#![feature(fs_try_exists)]

use clap::Parser;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

mod compiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the file to open
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    output: String,
}

fn lire_mots_fichier(path: &str) -> std::io::Result<Vec<String>> {
    let fichier = File::open(path)?;
    let lecteur = BufReader::new(fichier);

    let mut mots: Vec<String> = Vec::new();

    for ligne in lecteur.lines() {
        let ligne = ligne?;
        for mot in ligne.split_whitespace() {
            mots.push(mot.to_string());
        }
    }

    Ok(mots)
}

pub fn compile(path: &str) -> Vec<u8> {
    let file = lire_mots_fichier(path).unwrap();

    let tokens = compiler::lexer::lex(file);

    #[cfg(debug_assertions)]
    println!("{:?}", tokens);

    let code = compiler::generator::generate(tokens);

    #[cfg(debug_assertions)]
    println!("{:?}", code);

    code
}

fn main() {
    let args = Args::parse();

    let code = compile(args.file.as_str());

    let mut file = File::create(args.output.as_str()).unwrap();

    // Step 3: Write the binary data to the file
    file.write_all(&code).unwrap();
}
