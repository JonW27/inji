use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main(){

    let f = File::open("cmd.dw");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Unable to open file: {:?}", error)
        },
    };
    
    let reader = BufReader::new(f);

    let lines = reader.lines().map(|l| l.unwrap());
    for line in lines{
        // do the parsing here
        if line == "line"{
            println!("line cmd");
        }
        println!("{}", line);
    }

}
