use std::{io, process};

use structopt::StructOpt;

mod cow;
mod fortune;

fn main() {
    let opt: Opt = Opt::from_args();

    let cow_file = match opt.cow_file {
        Some(ref cow_file) => cow_file,
        None => "default",
    };
    let cow = match cow::Cow::new(cow_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Cannot open cow file: {}", e);
            process::exit(1);
        }
    };
    let text = if opt.fortune {
        select_fortune()
    } else if !opt.text.is_empty() {
        opt.text.join(" ")
    } else {
        read_text_from_stdin()
    };

    println!("{}", cow.print(&text, opt.thoughts, &opt.eyes, &opt.tongue))
}

fn select_fortune() -> String {
    let fortunes = match fortune::Fortunes::load_database() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot read fortune database: {}", e);
            process::exit(1);
        }
    };
    match fortunes.select() {
        Ok(Some(f)) => f,
        Ok(None) => {
            eprintln!("No fortunes in the database");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Cannot read a fortune: {}", e);
            process::exit(1);
        }
    }
}

fn read_text_from_stdin() -> String {
    io::stdin()
        .lines()
        .map(|l| match l {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot read from stdin: {}", e);
                process::exit(1);
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "f", help = "Cow picture file to use")]
    cow_file: Option<String>,
    #[structopt(short = "t", help = "Use thoughts rather than speech bubble")]
    thoughts: bool,
    #[structopt(short = "e", default_value = "oo", help = "Eyes string")]
    eyes: String,
    #[structopt(short = "T", default_value = "  ", help = "Tongue string")]
    tongue: String,
    #[structopt(short = "F", help = "Pick a random fortune for the text")]
    fortune: bool,
    text: Vec<String>,
}
