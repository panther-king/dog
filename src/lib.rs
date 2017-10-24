extern crate getopts;

use getopts::{Fail, Options};

const PROGRAM_NAME: &str = "dog";

type DogResult<T> = Result<T, DogError>;

#[derive(Debug)]
pub enum DogError {
    BudFood(Fail),
    EmptyFood,
}

impl From<Fail> for DogError {
    fn from(err: Fail) -> DogError {
        DogError::BudFood(err)
    }
}

pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn new(args: Vec<String>) -> Self {
        Args { args: args }
    }

    pub fn args(&self) -> Vec<String> {
        if self.is_cli() {
            self.args
                .iter()
                .skip(1)
                .map(|arg| arg.clone())
                .collect()
        } else {
            self.args
                .iter()
                .map(|arg| arg.clone())
                .collect()
        }
    }

    fn is_cli(&self) -> bool {
        self.args[0] == PROGRAM_NAME
    }
}

pub struct Dog {
    foods: Vec<String>,
}

impl Dog {
    pub fn foods(args: Args) -> DogResult<Dog> {
        let foods = args.args();
        if foods.is_empty() {
            return Err(DogError::EmptyFood);
        }

        Ok(Dog { foods: foods })
    }

    pub fn run(&self) -> DogResult<()> {
        let mut options = Options::new();
        options.optflag("h", "help", "print this help menu");
        options.optflag("", "dry-run", "what's dog eat");

        let matches = options.parse(&self.foods)?;
        if matches.opt_present("h") || matches.free.is_empty() {
            return self.usage();
        }
        if matches.opt_present("dry-run") {
            return self.dry_run();
        }

        self.eat()
    }

    fn dry_run(&self) -> DogResult<()> {
        Ok(())
    }

    fn eat(&self) -> DogResult<()> {
        Ok(())
    }

    fn usage(&self) -> DogResult<()> {
        Ok(())
    }
}
