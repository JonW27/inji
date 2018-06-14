#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

extern crate clap;
use clap::{App};

//use std::process::Command;

mod display;
mod draw;
mod matrix;
mod parser;

fn main() {
    let matches = App::new("inji")
        .version("0.1.alpha")
        .about("A primitive graphics engine.")
        .author("Jonathan Wong <jcwong@protonmail.com>")
        .args_from_usage("-o, --output=[FILE] 'Sets an optional log output file'")
        .get_matches();
    
    if let Some(o) = matches.value_of("o") {
        //let mut log = File::create(o.to_string()+".log").expect("Logfile could not be created. Check to see if name already taken.");
        println!("Logging to file {}.log", o);
        println!("This function doesn't work right now. I will implement slog to do both stdout and logfile redirection soon.");
    }

    let screen = display::new_screen(500, 500);
    let stack : Vec<matrix::Matrix> = Vec::new();
    let zb = display::new_zbuffer(500, 500); // xres + yres

    parser::parse("cmd.dw", stack, screen, zb);
    // display(screen);
    // save_extension(screen, 'img.png');
}
