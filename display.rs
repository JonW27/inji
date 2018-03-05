use std::fs::File;
use std::io::Write;
use std::vec::Vec;
use std::process::{Command, Stdio};
use std::error::Error;

const XRES:i64 = 500;
const YRES:i64 = 500;
const MAX_COLOR:i64 = 255;
const RED:usize = 0;
const GREEN:usize = 1;
const BLUE:usize = 2;

static DEFAULT_COLOR : [i64; 3] = [255, 255, 255];

pub fn new_screen(width : i64, height : i64) -> Vec<Vec<[i64; 3]>>{
    let mut screen : Vec<Vec<[i64; 3]>> = Vec::new();
    for y in 0..height {
        let tmp = y as usize;
        let mut row : Vec<[i64; 3]> = Vec::new();
        screen.push(row);
        for _x in 0..width {
            &screen[tmp].push(DEFAULT_COLOR);
        }
    }
    return screen;
}

pub fn plot( screen : &mut Vec<Vec<[i64; 3]>>, color : [i64; 3], x : i64, y : i64){
    let newy : i64 = YRES - 1 - y;
    if x >= 0 &&  x < XRES && newy >= 0 && newy < YRES {
        let ny = newy as usize;
        let nx = x as usize;
        screen[ny][nx][0] = color[0];
        screen[ny][nx][1] = color[1];
        screen[ny][nx][2] = color[2];
    }
}

pub fn clear_screen( screen : &mut Vec<Vec<[i64; 3]>> ){
    for y in 0..screen.len() {
        let ny = y as usize;
        for x in 0..screen[ny].len() {
            let nx = x as usize;
            screen[ny][nx][0] = DEFAULT_COLOR[0];
            screen[ny][nx][1] = DEFAULT_COLOR[1];
            screen[ny][nx][2] = DEFAULT_COLOR[2];
        }
    }
}

pub fn save_ppm( screen : &mut Vec<Vec<[i64; 3]>>, fname : String<>){
    let mut pic = File::create(fname).expect("File could not be created. Check to see if name already taken.");
    let mut ppm  = "P3 \n500 500 \n255\n".to_string();
    for y in 0..screen.len() {
        let mut row = String::new();
        let ny = y as usize;
        for x in 0..screen[ny].len(){
            let nx = x as usize;
            let pixel = screen[ny][nx];
            let rgb = pixel[RED].to_string() + " " + &pixel[GREEN].to_string() + " " + &pixel[BLUE].to_string();
            row.push_str(&rgb);
            row.push_str(" ");
        }
        ppm.push_str(&row);
        ppm.push_str("\n");
    }
    pic.write(ppm.as_bytes()).expect("Could not write to file");
    drop(ppm);
}

/*
fn save_extension( screen : &mut Vec<Vec<[i64; 3]>>, fname : String<>){

}
*/

pub fn display( screen : &mut Vec<Vec<[i64; 3]>>){
    let mut buf = "P3 \n500 500 \n255\n".to_string();
    for y in 0..screen.len() {
        let mut row = String::new();
        let ny = y as usize;
        for x in 0..screen[ny].len(){
            let nx = x as usize;
            let pixel = screen[ny][nx];
            let rgb = pixel[RED].to_string() + " " + &pixel[GREEN].to_string() + " " + &pixel[BLUE].to_string();
            row.push_str(&rgb);
            row.push_str(" ");
        }
        buf.push_str(&row);
        buf.push_str("\n");
    }
    let process = match Command::new("display") // why does it not wait ¯\_(ツ)_/¯
                                .stdin(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn display: {}", why.description()),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(buf.as_bytes()) {
        Err(why) => panic!("couldn't write to display stdin: {}",
                           why.description()),
        Ok(_) => println!("Displaying screen."),
    }
}
