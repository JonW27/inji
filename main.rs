#![allow(dead_code)]
#![allow(non_snake_case)]

mod display;
mod draw;
mod matrix;

fn main() {
    let mut screen = display::new_screen(500, 500);
    let color = [0, 255, 0];

    let mut m1 = matrix::new_matrix(4, 4);

    println!("================ Testing Edge Addition ================");
    draw::add_edge(&mut m1, 0, 0, 0, 1, 1, 0);
    println!("Test 1:");
    matrix::print_matrix(m1);

    let m2 = matrix::new_matrix(4, 4);
    let mut e = matrix::new_matrix(4, 4);
    let t = matrix::new_matrix(4, 4);

    draw::add_edge(&mut e, 0, 0, 0, 1, 1, 0);
    draw::add_edge(&mut e, 0, 1, 0, 1, 2, 0);
    draw::add_edge(&mut e, 0, 1, 0, 1, 2, 0);
    draw::add_edge(&mut e, 1, 1, 0, 2, 2, 0);
    draw::add_edge(&mut e, 3, 3, 0, 1, 0, 0);

    draw::draw_lines(e, &mut screen, color);

    display::save_ppm(&mut screen, "coolio.ppm".to_string());
    // display(screen);
    // save_extension(screen, 'img.png');
}
