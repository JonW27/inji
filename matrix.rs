
pub struct Matrix{
    pub m : Vec<Vec<f64>>,
    rows : i64,
    cols : i64,
    pub lastcol : i64,
}

pub fn make_bezier() -> Matrix {
    let mut trans : Matrix = new_matrix(4,4);

    trans.m[0][0] = -1.;
    trans.m[0][1] = 3.;
    trans.m[0][2] = -3.;
    trans.m[0][3] = 1.;
    trans.m[1][0] = 3.;
    trans.m[1][1] = -6.;
    trans.m[1][2] = 3.;
    trans.m[1][3] = 0.;
    trans.m[2][0] = -3.;
    trans.m[2][1] = 3.;
    trans.m[2][2] = 0.;
    trans.m[2][3] = 0.;
    trans.m[3][0] = 1.;
    trans.m[3][1] = 0.;
    trans.m[3][2] = 0.;
    trans.m[3][3] = 0.;
    trans.lastcol = 4;
    trans
}

pub fn make_hermite() -> Matrix {
    let mut trans : Matrix = new_matrix(4,4);

    trans.m[0][0] = 2.;
    trans.m[0][1] = -2.;
    trans.m[0][2] = 1.;
    trans.m[0][3] = 1.;
    trans.m[1][0] = -3.;
    trans.m[1][1] = 3.;
    trans.m[1][2] = -2.;
    trans.m[1][3] = -1.;
    trans.m[2][0] = 0.;
    trans.m[2][1] = 0.;
    trans.m[2][2] = 1.;
    trans.m[2][3] = 0.;
    trans.m[3][0] = 1.;
    trans.m[3][1] = 0.;
    trans.m[3][2] = 0.;
    trans.m[3][3] = 0.;
    trans.lastcol = 4;
    trans
}

pub fn generate_curve_coefs(p1 : f64, p2 : f64, p3 : f64, p4 : f64, type_curve : i64) -> Matrix {
    let mut t : Matrix = new_matrix(4, 1);
    t.m[0][0] = p1;
    t.m[1][0] = p2;
    t.m[2][0] = p3;
    t.m[3][0] = p4;
    t.lastcol = 1;
    if type_curve == 0 {
        matrix_mult(&mut make_hermite(), &mut t);
    } else {
        matrix_mult(&mut make_bezier(), &mut t);
    }
    t
}


pub fn make_translate(x : f64, y : f64, z : f64) -> Matrix{
    let mut trans : Matrix = new_matrix(4,4);
    ident(&mut trans);
    trans.m[0][3] = x;
    trans.m[1][3] = y;
    trans.m[2][3] = z;
    trans
}

pub fn make_scale(x : f64, y : f64, z : f64) -> Matrix{
    let mut trans : Matrix = new_matrix(4,4);
    ident(&mut trans);
    trans.m[0][0] = x;
    trans.m[1][1] = y;
    trans.m[2][2] = z;
    trans
}

pub fn make_rotX(theta : f64) -> Matrix{
    let mut trans : Matrix = new_matrix(4,4);
    let t = theta.to_radians();
    ident(&mut trans);
    trans.m[1][1] = t.cos();
    trans.m[1][2] = -1. * t.sin();
    trans.m[2][1] = t.sin();
    trans.m[2][2] = t.cos();
    trans
}

pub fn make_rotY(theta : f64) -> Matrix{
    let mut trans : Matrix = new_matrix(4,4);
    let t = theta.to_radians();
    ident(&mut trans);
    trans.m[0][0] = t.cos();
    trans.m[2][0] = -1. * t.sin();
    trans.m[0][2] = t.sin();
    trans.m[2][2] = t.cos();
    trans
}

pub fn make_rotZ(theta : f64) -> Matrix{
    let mut trans: Matrix = new_matrix(4,4);
    let t = theta.to_radians();
    ident(&mut trans);
    trans.m[0][0] = t.cos();
    trans.m[0][1] = -1. * t.sin();
    trans.m[1][0] = t.sin();
    trans.m[1][1] = t.cos();
    trans
}

pub fn print_matrix( m : & Matrix){
    // iterators!
    let m_cols = m.m.iter();

    for m_col in m_cols {
        let m_rows = m_col.iter();
        for val in m_rows {
            print!("{} ", val);
        }
        println!("");
    }
}

// the below method assumes a square matrix
pub fn ident(m : &mut Matrix){
    let len = m.m.len();

    for m_col in 0..len {
        for m_row in 0..len {
            if m_col == m_row {
                m.m[m_col][m_row] = 1.;
            }
            else {
                m.m[m_col][m_row] = 0.;
            }
        }
    }
    m.lastcol = m.cols;
}

pub fn matrix_mult(a : &mut Matrix, b : &mut Matrix){
    let a_refs = &a.m;
    let b_refs = &mut b.m;

    let mut tmp = new_matrix(4, 1);


    for j in 0..b.lastcol{
        let j = j as usize;
        for i in 0..b.rows{ let i = i as usize; tmp.m[i][0] = b_refs[i][j];}
        for i in 0..b.rows{
            let i = i as usize;
            b_refs[i][j] = a_refs[i][0] * tmp.m[0][0] + a_refs[i][1] * tmp.m[1][0] + a_refs[i][2] * tmp.m[2][0] + a_refs[i][3] * tmp.m[3][0];
        }
    }

}

pub fn new_matrix(rows : i64, cols: i64) -> Matrix {
    let mut tmp : Vec<Vec<f64>> = Vec::new();
    for y in 0..rows {
        let tmp_a = y as usize;
        let mut col : Vec<f64> = Vec::new();
        tmp.push(col);
        for _x in 0..cols {
            &tmp[tmp_a].push(0.);
        }
    }
    Matrix {
        m : tmp,
        rows,
        cols,
        lastcol : 0,
    }
}

/*pub fn setVal(row: i64, col: i64){

}*/

// no need to free matrix since rust takes care of it with scopes!

// vecs are mutable, so they can grow!

// copying a matrix can be easily done with .clone()!
