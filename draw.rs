use display;
use std::f64::consts::PI;

pub use matrix::Matrix;
pub use matrix::generate_curve_coefs;

const HERMITE:i64 = 0;
const BEZIER:i64 = 1;

pub fn add_circle( points : &mut Matrix, cx : f64, cy : f64, cz : f64, r : f64, step : f64){
    let mut x : f64 = cx + r;
    let mut y : f64 = cy;
    let (mut x1, mut y1);
    let step = step as i64 * 100;
    for t in step..100 {
        let t = t as f64 / 100.;
        x1 = cx + r * (2. * PI * t).cos();
        y1 = cy + r * (2. * PI * t).sin();
        add_edge(points, x, y, cz, x1, y1, cz);
        x = x1;
        y = y1;
    }
}

pub fn add_curve( points : &mut Matrix, mut x0 : f64, mut y0 : f64, x1 : f64, y1 : f64, x2 : f64, y2 : f64, x3 : f64, y3 : f64, step : f64, curve_type : i64){
    let x_coefs = generate_curve_coefs(x0, x1, x2, x3, curve_type);
    let y_coefs = generate_curve_coefs(y0, y1, y2, y3, curve_type);
    let step = step as i64 * 100;
    for t in step..100 {
        let t = t as f64 / 100.;
        let x1 = x_coefs.m[0][0]*t.powf(3.) + x_coefs.m[1][0]*t.powf(2.)+x_coefs.m[2][0]*t + x_coefs.m[3][0];
        let y1 = y_coefs.m[0][0]*t.powf(3.) + y_coefs.m[1][0]*t.powf(2.)+y_coefs.m[2][0]*t + y_coefs.m[3][0];
        add_edge(points, x0, y0, 0., x1, y1, 0.);
        x0 = x1;
        y0 = y1;
    }
}

pub fn add_point( points : &mut Matrix, x : f64, y : f64, z : f64){
    if points.lastcol >= points.m[0].len() as i64 {
        points.m[0].push(x);
        points.m[1].push(y);
        points.m[2].push(z);
        points.m[3].push(1.);
    }
    else{
        let ind = points.lastcol as usize;
        points.m[0][ind] = x;
        points.m[1][ind] = y;
        points.m[2][ind] = z;
        points.m[3][ind] = 1.;
    }
    points.lastcol += 1;
}

pub fn add_edge( points : &mut Matrix, x0 : f64, y0 : f64, z0 : f64, x1 : f64, y1 : f64, z1 : f64){
    add_point(points, x0, y0, z0);
    add_point(points, x1, y1, z1);
}

pub fn draw_lines( points : &mut Matrix, s : &mut Vec<Vec<[i64; 3]>>, c : [i64; 3]){
    let m = &points.m;
    let mut past_point_x : f64 = 0.;
    let mut past_point_y : f64 = 0.;
    for i in 0..m[0].len(){
        if i % 2 != 0{
            draw_line(past_point_x as i64, past_point_y as i64, m[0][i] as i64, m[1][i] as i64, s, c);
        }
        past_point_x = m[0][i].clone();
        past_point_y = m[1][i].clone();
    }
}

pub fn draw_line( mut x0 : i64, mut y0 : i64, x1 : i64, y1 : i64, s : &mut Vec<Vec<[i64; 3]>>, c : [i64; 3] ){
    let (mx, my) = (x1 - x0, y1 - y0);
    if x0 > x1 { // we want the (x1,y1) pair to always be greater so we switch
        return draw_line(x1, y1, x0, y0, s, c);
    }
    let (A, B) = (my, -1 * (mx));
    let mut d : i64;
    if my > 0{ // first quadrant
        if mx > my  { // first octant
            d = 2*A + B;
            while x0 <= x1 {
                display::plot(s, c, x0, y0);
                if d > 0 {
                    y0 += 1;
                    d += 2*B;
                }
                x0 += 1;
                d += 2*A;
            }
        }
        else{ // second octant
            d = A + 2*B;
            while y0 <= y1 {
                display::plot(s, c, x0, y0);
                if d < 0 {
                    x0 += 1;
                    d += 2*A;
                }
                y0 += 1;
                d += 2*B;
            }
        }
    }
    else { // quadrant 4
        if mx + my < 0 { // octant 7, mx is always positive so my would need to be less to satisfy
            d = A - 2*B;
            while y0 >= y1 {
                display::plot(s, c, x0, y0);
                if d > 0 {
                    x0 += 1;
                    d += 2*A;
                }
                y0 -= 1;
                d -= 2*B;
            }
        }
        else { // octant 8
            d = 2*A - B;
            while x0 <= x1 {
                display::plot(s, c, x0, y0);
                if d < 0 {
                    y0 -= 1;
                    d -= 2*B;
                }
                x0 += 1;
                d += 2*A;
            }
        }
    }
}
