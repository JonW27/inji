
pub struct Matrix{
    pub m : Vec<Vec<i64>>,
    rows : i64,
    cols : i64,
    pub lastcol : i64,
}

pub fn make_translate(x : f64, y : f64, z : f64){
    
}

pub fn make_scale(x : f64, y : f64, z : f64){

}

pub fn make_rotX(theta : f64){

}

pub fn make_rotY(theta : f64){

}

pub fn make_rotZ(theta : f64){

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
                m.m[m_col][m_row] = 1;
            }
            else {
                m.m[m_col][m_row] = 0;
            }
        }
    }
}

pub fn matrix_mult(a : &mut Matrix, b : &mut Matrix) -> Matrix{

    let a_refs = &a.m;
    let b_refs = &b.m;

    let mut c = new_matrix(a_refs.len() as i64, b_refs.len() as i64);

    for i in 0..a_refs.len(){
        for j in 0..b_refs[0].len(){
            let mut sum = 0;
            for k in 0..a_refs.len() { // usually 3
                sum += a_refs[i][k] * b_refs[k][j];
            }
            c.m[i][j] = sum;
        }
    }
    c
}

pub fn new_matrix(rows : i64, cols: i64) -> Matrix {
    let mut tmp : Vec<Vec<i64>> = Vec::new();
    for y in 0..cols {
        let tmp_a = y as usize;
        let mut row : Vec<i64> = Vec::new();
        tmp.push(row);
        for _x in 0..rows {
            &tmp[tmp_a].push(0);
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
