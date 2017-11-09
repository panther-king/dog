extern crate getopts;

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use getopts::{Fail, Options};

use self::DogError::*;
use self::TastingError::*;

const PROGRAM_NAME: &str = "dog";

pub type BadFood = BTreeMap<String, TastingError>;
pub type DogResult<T> = Result<T, DogError>;

#[derive(Debug)]
pub enum DogError {
    EmptyFood,
    RunAway(Fail),
    Uneatable(BadFood),
}

impl From<Fail> for DogError {
    fn from(err: Fail) -> DogError {
        DogError::RunAway(err)
    }
}

impl fmt::Display for DogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmptyFood => write!(f, "No targets specified"),
            RunAway(ref err) => write!(f, "{}", err),
            Uneatable(ref err) => {
                let errors = err.iter()
                    .map(|(k, v)| {
                        format!("{}: {}", k, v)
                    })
                    .collect::<Vec<String>>();
                write!(f, "{}", errors.join("\n"))
            }
        }
    }
}

#[derive(Debug)]
pub enum TastingError {
    Confused,
    NotAllowed,
    NotExist,
    NotFood,
}

impl fmt::Display for TastingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Confused => write!(f, "dog is confused"),
            NotAllowed => write!(f, "permission denied."),
            NotExist => write!(f, "does not exist."),
            NotFood => write!(f, "is not legurar-file"),
        }
    }
}

#[derive(Debug)]
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
        if self.args.len() == 0 {
            return false;
        }

        self.args[0] == PROGRAM_NAME
    }
}

#[derive(Debug)]
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

        let matches = options.parse(&self.foods)?;
        if matches.opt_present("h") || matches.free.is_empty() {
            return self.usage();
        }

        let tasting = self.wait();
        if tasting.len() != 0 {
            return Err(DogError::Uneatable(tasting));
        }

        Ok(self.eat())
    }

    fn eat(&self) {
        for food in self.foods.iter() {
            let _ = fs::File::create(food);
        }
    }

    fn taste(&self, path: &Path) -> Result<(), TastingError> {
        fs::metadata(path)
            .map_err(|e| {
                match e.kind() {
                    ErrorKind::PermissionDenied => NotAllowed,
                    ErrorKind::NotFound => NotExist,
                    _ => Confused
                }
            })
            .and_then(|m| {
                if m.is_dir() {
                    Err(NotFood)
                } else if m.permissions().readonly() {
                    Err(NotAllowed)
                } else {
                    Ok(())
                }
            })
    }

    fn usage(&self) -> DogResult<()> {
        Ok(())
    }

    fn wait(&self) -> BTreeMap<String, TastingError> {
        let mut foods = BTreeMap::new();
        for food in self.foods.iter() {
            let path = Path::new(&food);
            match self.taste(&path) {
                Ok(_) => continue,
                Err(e) => foods.insert(food.clone(), e),
            };
        }

        foods
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn foods() -> Vec<String> {
        vec!["a".into(), "b".into(), "c".into()]
    }

    #[test]
    fn test_args() {
        let args = Args::new(foods());

        assert_eq!(args.args(), foods());
    }

    #[test]
    fn test_cli_args() {
        let mut seed = foods();
        seed.insert(0, "dog".into());
        let args = Args::new(seed);

        assert_eq!(args.args(), foods());
    }

    #[test]
    fn test_empty_food() {
        let args = Args::new(vec![]);
        let dog = Dog::foods(args);

        assert!(dog.is_err());
    }

    #[test]
    fn test_uneatable() {
        let args = Args::new(foods());
        let dog = Dog::foods(args).unwrap();

        assert!(dog.run().is_err());
    }
}
