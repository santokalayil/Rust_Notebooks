use std::fs::File;
use std::io::prelude::*;


fn main() {

    let mut my_file = File::open("hello.txt").expect("Cannot open file!");

    let mut contents = String::new();
    my_file.read_to_string(&mut contents).expect("oops! cannot read");
    // above it should be mutable reference
    println!("file contents \n\n{}", contents);
}
