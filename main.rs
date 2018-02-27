#![allow(dead_code)]
#![allow(non_snake_case)]

mod display;
mod draw;
mod matrix;

fn main() {
    let mut screen = display::new_screen(500, 500);
    let color = [0, 255, 0];

    let mut m0 = matrix::new_matrix(4, 4);
    let mut m1 = matrix::new_matrix(4, 4);

    println!("<================ Testing Point Addition ================>");
    draw::add_point(&mut m0, 1, 1, 1);
    println!("Test 1, adding point to m0:");
    matrix::print_matrix(&m0);

    println!("<================ Testing Edge Addition ================>");
    draw::add_edge(&mut m1, 0, 0, 0, 1, 1, 0);
    println!("Test 1, adding edge to m1:");
    matrix::print_matrix(&m1);

    println!("<================ Testing Identity Matrix Conversion ================>");
    println!("Test 1, converting m1 to identity matrix:");
    matrix::ident(&mut m1);
    matrix::print_matrix(&m1);

    println!("<================ Testing Matrix Multiplication ================>");

    let mut e = matrix::new_matrix(4, 4);
    let mut t = matrix::new_matrix(4, 4);

    matrix::ident(&mut t);

    println!("Test 1, multiplying edge matrix by a transformation identity matrix:");

    println!("Transformation Matrix");
    matrix::print_matrix(&t);

    draw::add_edge(&mut e, 0, 0, 0, 1, 1, 0);
    draw::add_edge(&mut e, 0, 1, 0, 1, 2, 0);

    println!("Edge Matrix");
    matrix::print_matrix(&e);

    println!("Resultant Matrix");
    let ret = matrix::matrix_mult(&mut t, &mut e);
    matrix::print_matrix(&ret);

    println!("Test 2, multiplying edge matrix by a transformation matrix with scale 2:");
    println!("Transformation Matrix");
    let t_len = t.m.len();
    let t_len_1 = t.m[0].len();
    for col in 0..t_len{
        for row in 0..t_len_1{
            t.m[col][row] = 2;
        }
    }
    matrix::print_matrix(&t);

    println!("Edge Matrix");
    matrix::print_matrix(&e);

    println!("Resultant Matrix");
    let ret = matrix::matrix_mult(&mut t, &mut e);
    matrix::print_matrix(&ret);

    matrix::ident(&mut e);

    let colors : [[i64; 3]; 8] = [[255, 59, 48], [255, 149, 0], [255, 204, 0], [76, 217, 100], [90, 200, 250], [0, 122, 255], [88, 86, 214], [255, 45, 85]];

    let (x, mut y) = (250, 250);
    for i in 0..8 {
        let (mut x0, mut y0) = (x, y);
        for _j in 0..100 {
                draw::add_edge(&mut e, x0, y0, 0, x0+1, y0+5, 0);
                x0-= 1;
                y0-= 1;
        }
        //println!("{}, {}, {}", colors[i][0],  colors[i][1]);
        draw::draw_lines(&mut e, &mut screen, colors[i]); // colors wont change for some reason despite clear incrementing, idk why ¯\_(ツ)_/¯
        y -= 10;
    }

    //draw::add_edge(&mut e, 0, 20, 0, 100, 200, 0);
    draw::draw_line(0, 0, 400, 400, &mut screen, color);
    //draw::draw_lines(&mut e, &mut screen, color);

    display::save_ppm(&mut screen, "coolio.ppm".to_string());
    // display(screen);
    // save_extension(screen, 'img.png');
}
