extern crate dog;

use std::env;
use dog::{Args, Dog};

fn main() {
    let foods = Args::new(env::args().collect());
    let dog = Dog::foods(foods).expect("error");

    match dog.run() {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
