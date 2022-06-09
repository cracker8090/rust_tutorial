extern crate basic_lib;

use std::io::{Error, Read, ErrorKind};
use std::{error, io};

use basic_lib::my_string::play;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    play("my string test".to_string());
    println!("my test");
    let v = vec![1, 2, 4];
    println!("{:?}", v);
    // std::thread::spawn(move || {
    //     println!("Hello thread");
    // })
    // .join();
    test_hashmap();
    // let mut file = std::fs::File::open("file.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
    test_file()?;
    Ok(())
}

use std::collections::HashMap;

fn test_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("yellow"), 20);
    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("yellow")).or_insert(50);
    println!("{:?}", scores);
}

use std::fs::File;
use std::io::prelude::*;

fn test_file() -> io::Result<()> {
    // let mut f = File::open("../file.txt").unwrap();
    let f = File::open("file1.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("file1.txt") {
                Ok(fc) => fc,
                    // println!("my test create file1.txt");
                    // fc
                // },
                Err(e) => panic!("my test create file error,{:?}",e),
            }
        },
        Err(error) => panic!("my test another file error,{:?}", error),
    };
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}
