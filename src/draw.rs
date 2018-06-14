use display;
use std::f64::consts::PI;

pub use matrix::Matrix;
pub use matrix::{generate_curve_coefs, make_rotX, new_matrix};

const HERMITE:i64 = 0;
const BEZIER:i64 = 1;

type zbuffer = Vec<Vec<f64>>;

                        // polygonal matrix // triangle   // screen                    // zbuffer
pub fn scanline_convert(p : &mut Matrix, i : i64, s : &mut Vec<Vec<[i64; 3]>>, zb : &mut zbuffer){
    
    let i = i as usize;

    // let mut pol = &mut p.m[3*i..3*i+2]; // get vector polygon slice
    // above example doesn't work since we're using arrays of [[x:],[y:],[z:]] and thus need a general access to a subset of the 2d array, which neccesitates another 2d array

    // thus the way to go is for loop iteration to make it easier for it to be sorted
    let mut pol = Vec::new();
    
    let mut first_vec = Vec::new();
    let mut second_vec = Vec::new();
    let mut third_vec = Vec::new();

    for j in 0..3 {
        first_vec.push(p.m[j][i]);
        second_vec.push(p.m[j][i+1]);
        third_vec.push(p.m[j][i+2]);
    }

    pol.push(first_vec);
    pol.push(second_vec);
    pol.push(third_vec);

    // sort 2d array based off middle array
    pol.sort_by(|a, b| a[1].partial_cmp(&b[1]).expect("The polygon list y values were malformed and could not be processed."));
    // sorts the slice via a closure using the floating point version of cmp (partial_cmp), then unwrapping the Option so that the vec can be sorted

    let mut greater_than_middle = false;

    let xB : f64 = pol[0][0];
    let xM : f64 = pol[1][0];
    let xT : f64 = pol[2][0];

    let yB : f64 = pol[0][1];
    let yM : f64 = pol[1][1];
    let yT : f64 = pol[2][1];

    let zB : f64 = pol[0][2];
    let zM : f64 = pol[1][2];
    let zT : f64 = pol[2][2];

    let mut x0 : f64 = pol[0][0];
    let mut x1 : f64 = pol[0][0];

    let mut z0 : f64 = pol[0][2];
    let mut z1 : f64 = pol[0][2];

    let mut y : f64 = pol[0][1];

    let mut dx0 : f64 = if (yT - yB) > 0. {(xT - xB) / (yT - yB)} else {0.};
    let mut dx1 : f64 = if (yM - yB) > 0. {(xM - xB) / (yM - yB)} else {0.};
    let mut dz0 : f64 = if (yT - yB) > 0. {(zT - zB) / (yT - yB)} else {0.};
    let mut dz1 : f64 = if (yM - yB) > 0. {(zM - zB) / (yM - yB)} else {0.};

    while y < yT {
        // println!("looping x0{} y{} z0{} x1{} z1{} yT{} dx0{} dz0{}", x0, y, z0, x1, z1, yT, dx0, dz0);
        let color = [(53*i as i64)%255, (73*i as i64)%255, (83*i as i64)%255]; // black for now
        draw_line(x0 as i64, y as i64, z0, x1 as i64, y as i64, z1, s, zb, color);
        x0 += dx0;
        x1 += dx1;

        // calculate z values
        z0 += dz0;
        z1 += dz1;

        y += 1.;

        if !greater_than_middle && y >= yM { // has reached the second line
            greater_than_middle = !greater_than_middle;

            x1 = xM;
            z1 = zM;

            dx0 = if (yT - yM) > 0. {(xT - xM) / (yT - yM)} else {0.};
            dz0 = if (yT - yM) > 0. {(zT - zM) / (yT - yM)} else {0.};
        }
    }

    
    //let bottomVal = p1.min(p2.min(p3));
    //let topVal = p1.max(p2.max(p3));


}    

pub fn add_polygon(polygons : &mut Matrix, x0 : f64, y0 : f64, z0 : f64, x1 : f64, y1 : f64, z1 : f64, x2 : f64, y2 : f64, z2 : f64){
    add_point(polygons, x0, y0, z0);
    add_point(polygons, x1, y1, z1);
    add_point(polygons, x2, y2, z2);
}

pub fn draw_polygons(p : &mut Matrix, s : &mut Vec<Vec<[i64; 3]>>, zb : &mut zbuffer, c : [i64; 3], to_backface_cull : bool){
    if p.lastcol < 3 {
        println!("Cannot draw a triangle-based polygon if three points are not supplied.");
    }
    let (l, mut k) = (p.lastcol - 2, 0);
    while k < l {
        // x0, y0, x1, y1, s, c
        let i = k as usize;
        let n : [f64; 3] = get_surface_normal(p, i);
        if n[2] > 0. || !to_backface_cull {
            // draw_line(p.m[0][i] as i64, p.m[1][i] as i64, p.m[0][i+1] as i64, p.m[1][i+1] as i64, s, c);
            // draw_line(p.m[0][i+1] as i64, p.m[1][i+1] as i64, p.m[0][i+2] as i64, p.m[1][i+2] as i64, s, c);
            // draw_line(p.m[0][i+2] as i64, p.m[1][i+2] as i64, p.m[0][i] as i64, p.m[1][i] as i64, s, c);
            scanline_convert(p, i as i64, s, zb);
        }
        k += 3;
    }
}

pub fn get_surface_normal(polygons : &mut Matrix, point_index : usize) -> [f64; 3]{
    let (mut a, mut b, mut n) = ([0., 0., 0.], [0., 0., 0.], [0., 0., 0.]);

    // get the triangle sides
    // side a
    a[0] = polygons.m[0][point_index+1] - polygons.m[0][point_index]; // individual vector component x of side a
    a[1] = polygons.m[1][point_index+1] - polygons.m[1][point_index];
    a[2] = polygons.m[2][point_index+1] - polygons.m[2][point_index];

    // side b
    b[0] = polygons.m[0][point_index+2] - polygons.m[0][point_index];
    b[1] = polygons.m[1][point_index+2] - polygons.m[1][point_index];
    b[2] = polygons.m[2][point_index+2] - polygons.m[2][point_index];
    
    /* multiply out cross product
            A * B = < AyBz - AzBx, AzBx - AxBz, AxBy - AyBx>
    */

    n[0] = a[1]*b[2] - a[2]*b[1];
    n[1] = a[2]*b[0] - a[0]*b[2];
    n[2] = a[0]*b[1] - a[1]*b[0];

    n
} 

pub fn add_box(edges : &mut Matrix, x : f64, y :f64, z : f64, width: f64, height: f64, depth: f64){
    // front face
    add_polygon(edges, x, y, z, x, y-height, z, x+width, y-height, z);
    add_polygon(edges, x, y, z, x+width, y-height, z, x+width, y, z);

    // back face
    add_polygon(edges, x+width, y, z-depth, x+width, y-height, z-depth, x, y-height, z-depth);
    add_polygon(edges, x+width, y, z-depth, x, y-height, z-depth, x, y, z-depth);

    // top face
    add_polygon(edges, x, y, z-depth, x, y, z, x+width, y, z);
    add_polygon(edges, x, y, z-depth, x+width, y, z, x+width, y, z-depth);

    // bottom face
    add_polygon(edges, x, y-height, z, x+width, y-height, z-depth, x+width, y-height, z);
    add_polygon(edges, x, y-height, z, x, y-height, z-depth, x+width, y-height, z-depth);
    
    // left face
    add_polygon(edges, x, y, z-depth, x, y-height, z, x, y, z);
    add_polygon(edges, x, y, z-depth, x, y-height, z-depth, x, y-height, z);
    
    // right face
    add_polygon(edges, x+width, y, z, x+width, y-height, z, x+width, y-height, z-depth);
    add_polygon(edges, x+width, y, z, x+width, y-height, z-depth, x+width, y, z-depth);
    
}

pub fn add_sphere(edges : &mut Matrix, cx : f64, cy : f64, cz : f64, r : f64, step : i64){
    
    let sphere : Matrix = generate_sphere(cx, cy, cz, r, step);
    for i in 0..step{
        for j in 0..step{
            let t0 = i * (step+1) + j;
            let t1 = t0 + 1;
            let t2 = (t1 + step+1) % ((step + 1) * step);
            let t3 = (t0 + step+1) % ((step + 1) * step);
            let p0 = t0 as usize;
            let p1 = t1 as usize;
            let p2 = t2 as usize;
            let p3 = t3 as usize;
            add_polygon(edges, sphere.m[0][p0], sphere.m[1][p0], sphere.m[2][p0], sphere.m[0][p1], sphere.m[1][p1], sphere.m[2][p1], sphere.m[0][p2], sphere.m[1][p2], sphere.m[2][p2]);
            add_polygon(edges, sphere.m[0][p0], sphere.m[1][p0], sphere.m[2][p0], sphere.m[0][p2], sphere.m[1][p2], sphere.m[2][p2], sphere.m[0][p3], sphere.m[1][p3], sphere.m[2][p3]);
        }
    }
}

pub fn generate_sphere(cx :f64, cy : f64, cz : f64, r : f64, step : i64) -> Matrix {
    
    let mut sphere : Matrix = new_matrix(4, step);

    for i in 0..step { 
        let phi = i as f64 / step as f64;
        for j in 0..step+1 {
            let theta = j as f64 / step as f64;
            let x = r*(theta * PI).cos()+cx;
            let y = r*(theta * PI).sin()*(phi * 2. * PI).cos()+cy;
            let z = r*(theta * PI).sin()*(phi * 2. * PI).sin()+cz;
            add_point(&mut sphere, x, y, z);
        }
    }
    sphere
}

pub fn add_torus(edges : &mut Matrix, cx : f64, cy : f64, cz : f64, r1 : f64, r2 : f64, step : i64){
    let torus : Matrix = generate_torus(cx, cy, cz, r1, r2, step);
    for i in 0..step{
        for j in 0..step{
             let t0 = i * (step+1) + j;
            let t1 = t0 + 1;
            let t2 = (t1 + step+1) % ((step + 1) * step);
            let t3 = (t0 + step+1) % ((step + 1) * step);
            let p0 = t0 as usize;
            let p1 = t1 as usize;
            let p2 = t2 as usize;
            let p3 = t3 as usize;
            add_polygon(edges, torus.m[0][p0], torus.m[1][p0], torus.m[2][p0], torus.m[0][p2], torus.m[1][p2], torus.m[2][p2], torus.m[0][p1], torus.m[1][p1], torus.m[2][p1]); // flipped due to oppo directions from sphere (front instead of back)
            add_polygon(edges, torus.m[0][p0], torus.m[1][p0], torus.m[2][p0], torus.m[0][p3], torus.m[1][p3], torus.m[2][p3], torus.m[0][p2], torus.m[1][p2], torus.m[2][p2]); // flipped due to oppo directions from sphere (front instead of back)
        }
    }
    // lol this fxn is weird
}

pub fn generate_torus(cx :f64, cy : f64, cz : f64, r1 : f64, r2 : f64, step : i64) -> Matrix {
    
    let mut torus : Matrix = new_matrix(4, step);

    for i in 0..step { 
        let theta = i as f64 / step as f64 * 2. * PI;
        for j in 0..step+1 {
            let phi = j as f64 / step as f64 * 2. * PI;
            let x = phi.cos()*(r1*theta.cos()+r2)+cx;
            let y = r1*theta.sin()+cy;
            let z = (r1*theta.cos()+r2)*phi.sin()+cz;
            add_point(&mut torus, x, y, z);
        }
    }
    torus
}    


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

pub fn draw_lines( points : &mut Matrix, s : &mut Vec<Vec<[i64; 3]>>, zb : &mut zbuffer, c : [i64; 3]){
    let m = &points.m;
    let mut past_point_x : f64 = 0.;
    let mut past_point_y : f64 = 0.;
    let mut past_point_z : f64 = 0.;
    for i in 0..m[0].len(){
        if i % 2 != 0{
            draw_line(past_point_x as i64, past_point_y as i64, past_point_z, m[0][i] as i64, m[1][i] as i64, m[2][i], s, zb, c);
        }
        past_point_x = m[0][i].clone();
        past_point_y = m[1][i].clone();
        past_point_z = m[2][i].clone();
    }
}

pub fn draw_line( mut x0 : i64, mut y0 : i64, mut z0 : f64,  x1 : i64, y1 : i64, mut z1 : f64, s : &mut Vec<Vec<[i64; 3]>>, zb : &mut zbuffer, c : [i64; 3] ){
    if x0 > x1 { return draw_line(x1, y1, z1, x0, y0, z0, s, zb, c); }
    let (mut d, mut A, mut B, mut dy_east, mut dy_northeast, mut dx_east, mut dx_northeast, mut d_east, mut d_northeast, mut loop_start, mut loop_end) : (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64);
    let (mut wide, mut tall) = (false, false);
    let (mut distance, mut dz) : (f64, f64);
    A = 2 * (y1 - y0);
    B = -2 * (x1 - x0);
    if (x1 - x0).abs() >= (y1 - y0).abs() { //octant 1/8
        wide = true;
        loop_start = x0;
        loop_end = x1;
        dx_east = 1;
        dx_northeast = 1;
        dy_east = 0;
        d_east = A;
        if A > 0 { //octant 1
            d = A + B/2;
            dy_northeast = 1;
            d_northeast = A + B;
        }
        else { //octant 8
            d = A - B/2;
            dy_northeast = -1;
            d_northeast = A - B;
        }
    }//end octant 1/8
    else { //octant 2/7
        tall = true;
        dx_east = 0;
        dx_northeast = 1;
        if A > 0 {     //octant 2
            d = A/2 + B;
            dy_east = 1;
            dy_northeast = 1;
            d_northeast = A + B;
            d_east = B;
            loop_start = y0;
            loop_end = y1;
        }
        else {     //octant 7
            d = A/2 - B;
            dy_east = -1;
            dy_northeast = -1;
            d_northeast = A - B;
            d_east = -1 * B;
            loop_start = y1;
            loop_end = y0;
        }
    }
    distance = ((loop_end - loop_start) as f64).abs();
    dz = if distance > 0. {z1 - z0 / distance} else{0.};
    while loop_start < loop_end {
        display::plot( s, zb, c, x0, y0, z0);
        if (wide && ((A > 0 && d > 0) || (A < 0 && d < 0))) || (tall && ((A > 0 && d < 0 ) || (A < 0 && d > 0) )) {
            x0+=dx_northeast;
            y0+=dy_northeast;
            d+=d_northeast;
        }
        else {
            x0+=dx_east;
            y0+=dy_east;
            d+=d_east;
        }
        z0+=dz;
        loop_start += 1;
    }
        display::plot( s, zb, c, x1, y1, z1 );
}