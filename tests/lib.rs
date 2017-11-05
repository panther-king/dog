extern crate dog;

use std::fs;
use std::io::prelude::*;

use dog::*;

fn create(path: &str) {
    if path.contains(".") {
        let mut f = fs::File::create(path).unwrap();
        let _ = f.write_all(path.as_bytes());
    } else {
        fs::DirBuilder::new().create(path).unwrap();
    }
}

#[test]
fn test_single_file() {
    let filename = "test.txt";
    create(filename);

    let args = Args::new(vec![filename.into()]);
    let dog = Dog::foods(args).unwrap();

    assert!(dog.run().is_ok());
    assert_eq!(0, fs::metadata(filename).unwrap().len());

    fs::remove_file(filename).unwrap();
}

#[test]
fn test_multi_files() {
    let filename1 = "test1.txt";
    create(filename1);

    let filename2 = "test2.txt";
    create(filename2);

    let args = Args::new(vec![filename1.into(), filename2.into()]);
    let dog = Dog::foods(args).unwrap();

    assert!(dog.run().is_ok());
    assert_eq!(0, fs::metadata(filename1).unwrap().len());
    assert_eq!(0, fs::metadata(filename2).unwrap().len());

    fs::remove_file(filename1).unwrap();
    fs::remove_file(filename2).unwrap();
}

#[test]
fn test_dir() {
    let dirname = "test_dir";
    create(dirname);

    let args = Args::new(vec![dirname.into()]);
    let dog = Dog::foods(args).unwrap();

    assert!(dog.run().is_err());

    fs::remove_dir(dirname).unwrap();
}

#[test]
fn test_include_dir() {
    let filename = "test3.txt";
    create(filename);

    let dirname = "test_dir2";
    create(dirname);

    let args = Args::new(vec![filename.into(), dirname.into()]);
    let dog = Dog::foods(args).unwrap();

    assert!(dog.run().is_err());

    fs::remove_file(filename).unwrap();
    fs::remove_dir(dirname).unwrap();
}
