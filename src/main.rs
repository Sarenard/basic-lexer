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

fn main() {
    let args = Args::parse();

    let file = lire_mots_fichier(args.file.as_str()).unwrap();

    let tokens = compiler::lexer::lex(file);

    println!("{:?}", tokens);

    let code = compiler::generator::generate(tokens);

    println!("{:?}", code);

    let mut file = File::create(args.output.as_str()).unwrap();

    // Step 3: Write the binary data to the file
    file.write_all(&code).unwrap();
}
