use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub use matrix::*;
pub use draw::*;
pub use display::{save_ppm, clear_screen, display};

const STEP:f64 = 0.01;
const STEP2:i64 = 10;
const HERMITE:i64 = 0;
const BEZIER:i64 = 1;
const IS_BACKFACE_CULLED:bool = true;

pub fn parse(f_name : &str, mut t : Matrix, mut e : Matrix, mut p : Matrix, mut s : Vec<Vec<[i64; 3]>>){

    let f = File::open(f_name);

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Unable to open file: {:?}", error)
        },
    };

    println!("<================ Parsing File {} ================>", f_name);

    let reader = BufReader::new(f);

    let lines : Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut cnt = 0;
    while cnt < lines.len(){
        let line = &lines[cnt];
        if line == "line"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            add_edge(&mut e, args[0], args[1], args[2], args[3], args[4], args[5]);
            println!("line {:?}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "ident"{
            ident(&mut t);
            println!("ident");
            cnt+= 1;
        } else if line == "clear"{
            e.lastcol = 0;
            p.lastcol = 0;
            println!("clear edge matrix");
            cnt+= 1;
        } else if line == "scale"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            matrix_mult(&mut make_scale(args[0], args[1], args[2]), &mut t);
            println!("scale {}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "move"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            matrix_mult(&mut make_translate(args[0], args[1], args[2]), &mut t);
            println!("move {}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "rotate"{
            let args = lines[cnt+1].split(" ").collect::<Vec<&str>>();
            if args[0] == "x"{
                println!("\nRotating Frame about x at {} degrees", args[1]);
                matrix_mult(&mut make_rotX(args[1].parse::<f64>().unwrap()), &mut t);
            } else if args[0] == "y"{
                println!("\nRotating Frame about y at {} degrees", args[1]);
                matrix_mult(&mut make_rotY(args[1].parse::<f64>().unwrap()), &mut t);
            } else if args[0] == "z"{
                println!("\nRotating Frame about z at {} degrees", args[1]);
                matrix_mult(&mut make_rotZ(args[1].parse::<f64>().unwrap()), &mut t);
            } else {
                println!("Could not rotate due to no axis being specified");
            }
            println!("rotate {}", lines[cnt+1]);
            cnt += 2;
        } else if line == "circle"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing circle {} {} {} {}", args[0], args[1], args[2], args[3]);
            add_circle(&mut e, args[0], args[1], args[2], args[3], STEP);
            cnt += 2;
        } else if line == "hermite"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing hermite curve {} {} {} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            add_curve(&mut e, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], STEP, HERMITE);
            cnt += 2;
        } else if line == "bezier"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing bezier curve {} {} {} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            add_curve(&mut e, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], STEP, BEZIER);
            cnt += 2;
        } else if line == "box"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing box {} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5]);
            add_box(&mut p, args[0], args[1], args[2], args[3], args[4], args[5]);
            cnt += 2;
        } else if line == "sphere"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing sphere {} {} {} {}", args[0], args[1], args[2], args[3]);
            add_sphere(&mut p, args[0], args[1], args[2], args[3], STEP2);
            cnt += 2;
        } else if line == "torus"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing torus {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4]);
            add_torus(&mut p, args[0], args[1], args[2], args[3], args[4], STEP2); //   
            cnt += 2;
        } else if line == "apply"{
            //println!("\nTransformation matrix");
            //print_matrix(&t);
            //println!("\nEdge matrix");
            //print_matrix(&e);
            matrix_mult(&mut t, &mut e);
            matrix_mult(&mut t, &mut p);
            //println!("\nResultant matrix");
            //print_matrix(&e);
            println!("apply");
            cnt += 1;
        } else if line == "display"{
            clear_screen(&mut s);
            draw_lines(&mut e, &mut s, [0, 0, 0]);
            draw_polygons(&mut p, &mut s, [0, 0, 0], IS_BACKFACE_CULLED);
            display(&mut s);
            cnt += 1;
        } else if line == "save" {
            clear_screen(&mut s);
            draw_lines(&mut e, &mut s, [0, 0, 0]);
            draw_polygons(&mut p, &mut s, [0, 0, 0], IS_BACKFACE_CULLED);
            save_ppm(&mut s, lines[cnt+1].to_string());
            println!("save {:?}", lines[cnt+1]);
            cnt += 2;
        } else{
            println!("Unrecognized cmd {}. Moving on.", lines[cnt]);
            cnt += 1;
        }
    }
}
