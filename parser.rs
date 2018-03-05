use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub use matrix::*;
pub use draw::*;
pub use display::save_ppm;
pub use display::clear_screen;

pub fn parse(f_name : &str, mut t : Matrix, mut e : Matrix, mut s : Vec<Vec<[i64; 3]>>){

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
    while cnt < lines.len() - 1{
        let line = &lines[cnt];
        if line == "line"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            add_edge(&mut e, args[0], args[1], args[2], args[3], args[4], args[5]);
            println!("line {:?}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "ident"{
            ident(&mut t);
            println!("ident");
            cnt+= 1;
        } else if line == "scale"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            t = matrix_mult(&mut t, &mut make_scale(args[0], args[1], args[2]));
            println!("scale {}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "move"{
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            t = matrix_mult(&mut t, &mut make_translate(args[0], args[1], args[2]));
            println!("move {}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "rotate"{
            let args = lines[cnt+1].split(" ").collect::<Vec<&str>>();
            if args[0] == "x"{
                t = matrix_mult(&mut t, &mut make_rotX(args[1].parse::<f64>().unwrap()));
            } else if args[0] == "y"{
                t = matrix_mult(&mut t, &mut make_rotY(args[1].parse::<f64>().unwrap()));
            } else if args[0] == "z"{
                t = matrix_mult(&mut t, &mut make_rotZ(args[1].parse::<f64>().unwrap()));
            } else {
                println!("Could not rotate due to no axis being specified");
            }
            println!("rotate {}", lines[cnt+1]);
            cnt += 2;
        } else if line == "apply"{
            e = matrix_mult(&mut t, &mut e);
            println!("apply");
            cnt += 1;
        } else if line == "display"{
            clear_screen(&mut s);
            draw_lines(&mut e, &mut s, [0, 0, 0]);
            println!("display");
            cnt += 1;
        } else if line == "save" {
            save_ppm(&mut s, lines[cnt+1].to_string());
            println!("save {:?}", lines[cnt+1]);
            cnt += 1;
        } else{
            println!("Unrecognized cmd {}. Moving on.", lines[cnt]);
        }
    }
}
