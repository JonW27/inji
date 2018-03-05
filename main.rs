#![allow(dead_code)]
#![allow(non_snake_case)]

mod display;
mod draw;
mod matrix;
mod parser;

fn main() {
    let screen = display::new_screen(500, 500);
    let e = matrix::new_matrix(4, 4);
    let t = matrix::new_matrix(4, 4);

    parser::parse("cmd.dw", t, e, screen);
    // display(screen);
    // save_extension(screen, 'img.png');
}
