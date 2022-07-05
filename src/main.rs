mod cow;
mod fortune;

use std::process;

use structopt::StructOpt;

fn main() {
    let opt: Opt = Opt::from_args();

    let fortunes = match fortune::Fortunes::load_database() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot read fortune database: {}", e);
            process::exit(1);
        }
    };
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
    let text = if opt.text.len() != 0 {
        opt.text.join(" ")
    } else {
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
    };

    println!("{}", cow.print(&text, opt.thoughts, &opt.eyes, &opt.tongue))
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "f")]
    cow_file: Option<String>,
    #[structopt(short = "t")]
    thoughts: bool,
    #[structopt(short = "e", default_value = "oo")]
    eyes: String,
    #[structopt(short = "T", default_value = "  ")]
    tongue: String,
    text: Vec<String>,
}
