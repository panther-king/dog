//! # dogrun
//!
//! The dog eats everthing.
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use self::DogError::*;
use self::TastingError::*;

pub type BadFood = BTreeMap<String, TastingError>;
pub type DogResult<T> = Result<T, DogError>;

#[derive(Debug)]
pub enum DogError {
    /// No specified targets
    EmptyFood,
    /// Files of targets are not writable
    Uneatable(BadFood),
}

impl fmt::Display for DogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmptyFood => write!(f, "No targets specified"),
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
    /// Something is wrong
    Confused,
    /// Permission denied
    NotAllowed,
    /// File does not exist
    NotExist,
    /// Not a regular-file
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
    /// Arguments of specified
    args: Vec<String>,
}

impl Args {
    /// Create an `Args`.
    ///
    /// # Example
    ///
    /// ```
    /// use dogrun::Args;
    ///
    /// let args = Args::new(vec!["a".into(), "b".into()]);
    /// assert_eq!(args.args().len(), 1);
    /// ```
    pub fn new(args: Vec<String>) -> Self {
        let collect = args.iter()
            .skip(1)
            .map(|arg| arg.clone())
            .collect();

        Args { args: collect }
    }

    /// Get targets.
    ///
    /// # Example
    ///
    /// ```
    /// use dogrun::Args;
    ///
    /// let args = Args::new(vec!["a".into(), "b".into()]);
    /// assert_eq!(args.args()[0], "b");
    /// ```
    pub fn args(&self) -> Vec<String> {
        self.args.clone()
    }
}

#[derive(Debug)]
pub struct Dog {
    /// Targets of files
    foods: Vec<String>,
}

impl Dog {
    /// Create a `Dog`.
    ///
    /// # Example
    ///
    /// ```
    /// use dogrun::{Args, Dog};
    ///
    /// let args = Args::new(vec!["a".into(), "b".into()]);
    /// let dog = Dog::foods(args);
    /// assert!(dog.is_ok());
    /// ```
    pub fn foods(args: Args) -> DogResult<Dog> {
        let foods = args.args();
        if foods.is_empty() {
            Err(EmptyFood)
        } else {
            Ok(Dog { foods: foods })
        }
    }

    /// Execute a command.
    ///
    /// `Dog` returns an error when foods could not eat.
    ///
    /// - Does not exist
    /// - Permission denied
    /// - Directory
    /// - Read-only permission
    /// - Not a regular-file
    pub fn run(&self) -> DogResult<()> {
        let tasting = self.wait();
        match tasting.len() {
            0 => Ok(self.eat()),
            _ => Err(Uneatable(tasting)),
        }
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
        let foods = vec!["b".to_string(), "c".to_string()];

        assert_eq!(args.args(), foods);
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
