#![allow(dead_code)]
#![allow(non_snake_case)]

mod display;
mod draw;
mod matrix;

fn main() {
    let mut screen = display::new_screen(500, 500);
    let color = [0, 255, 0];

    let edges = matrix::new_matrix(4, 4);

    draw::draw_lines(edges, &mut screen, color);

    /*draw::draw_line(0, 0, 20, 20, &mut screen, color); // check
    draw::draw_line(25, 5, 38, 76, &mut screen, color); // check
    draw::draw_line(10, 20, 70, 20, &mut screen, [255, 0, 0]); // check
    draw::draw_line(100, 250, 200, 70, &mut screen, [255, 0, 0]); // not ok
    draw::draw_line(0, 10, 5, 200, &mut screen, [255, 255, 255]); // slope does not increase dramatically, not ok
    draw::draw_line(150, 150, 20, 20, &mut screen, [0, 0, 200]); // check
    draw::draw_line(7, 10, 100, 100, &mut screen, [255, 255, 255]); // check*/
    display::save_ppm(&mut screen, "coolio.ppm".to_string());
    // display(screen);
    // save_extension(screen, 'img.png');
}
