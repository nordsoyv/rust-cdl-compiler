extern crate cdl_core;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use cdl_core::compile;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.len() {
        1 => {
            println!("Try passing some arguments");
            return;
        }
        2 => {
            println!("one argument passed");
            filename = &args[1];
            println!("{}", filename)
        },
        _ => {
            println!("Many args passed");
            return;
        }
    }

    let path = Path::new(filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {} : {}", display, why),
        Ok(file) => file
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut cdl_script = String::new();
    match file.read_to_string(&mut cdl_script) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => print!("{} contains:\n{}", display, cdl_script),
    }


    let compiled = match compile(cdl_script){
        Err(e) => panic!("Error compiling {:?}", e),
        Ok(r) => r
    };

    println!("Compiled {:?}", compiled);

}
