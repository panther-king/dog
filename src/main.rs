extern crate dog;

use std::env;
use dog::{Args, Dog};

fn main() {
    let foods = Args::new(env::args().collect());

    match Dog::foods(foods) {
        Ok(dog) => dog.run().unwrap_or_else(|e| {
            println!("{}", e);
        }),
        Err(e) => {
            println!("{}", e);
        }
    }
}
