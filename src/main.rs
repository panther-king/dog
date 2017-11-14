extern crate getopts;

extern crate dogrun;

use std::{env, process};

use getopts::Options;

use dogrun::{Args, Dog};

const USAGE: &'static str = "\
    USAGE:
    dog path ...";

fn main() {
    let mut options = Options::new();
    options.optflag("h", "help", "print this help menu");

    let args = env::args().collect();
    let matches = options.parse(&args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    if matches.opt_present("h") || matches.free.is_empty() {
        println!("{}", USAGE);
        process::exit(0);
    }

    let foods = Args::new(args);
    let dog = Dog::foods(foods).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    match dog.run() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
}
