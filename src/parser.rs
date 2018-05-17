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

pub fn parse(f_name : &str, mut stack : Vec<Matrix>, mut s : Vec<Vec<[i64; 3]>>){

    let f = File::open(f_name);

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Unable to open file: {:?}", error)
        },
    };

    println!("<================ Parsing File {} ================>", f_name);

    if IS_BACKFACE_CULLED {
        println!("The option BACKFACE_CULLED is ENABLED.");
    }

    let reader = BufReader::new(f);

    let mut stack_top = new_matrix(4,4);
    ident(&mut stack_top);
    stack.push(stack_top);

    let lines : Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut cnt = 0;
    while cnt < lines.len(){
        let line = &lines[cnt];
        let mut tmp = new_matrix(4, 4);
        if line == "push"{
            match stack.clone().last() { // stack.clone().last()
                Some(x) => { stack.push(x.clone()); print_matrix(&mut x.clone())},
                None => println!("there's literally no top of the stack"),
            }
            println!("push");
            cnt+= 1;
        }
        else if line == "pop"{
            stack.pop();
            cnt += 1;
        }
        else if line == "line"{
            let last : usize = stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            add_edge(&mut tmp, args[0], args[1], args[2], args[3], args[4], args[5]);
            matrix_mult(&mut stack[last], &mut tmp);
            draw_lines(&mut tmp, &mut s, [0, 0, 0]);
            println!("line {:?}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "scale"{
            let last : usize = stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            let mut tmp = &mut make_scale(args[0], args[1], args[2]);
            matrix_mult(&mut stack[last], tmp);
            stack[last] = tmp.clone();
            println!("scale {}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "move"{
            let last : usize = stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            let mut tmp = &mut make_translate(args[0], args[1], args[2]);
            matrix_mult(&mut stack[last], tmp);
            stack[last] = tmp.clone();
            println!("move {}", lines[cnt+1]);
            cnt+= 2;
        } else if line == "rotate"{
            let args = lines[cnt+1].split(" ").collect::<Vec<&str>>();
            let last : usize = stack.len()-1;
            if args[0] == "x"{
                println!("\nRotating Frame about x at {} degrees", args[1]);
                let tmp = &mut make_rotX(args[1].parse::<f64>().unwrap());
                matrix_mult(&mut stack[last], tmp);
                stack[last] = tmp.clone();
            } else if args[0] == "y"{
                println!("\nRotating Frame about y at {} degrees", args[1]);
                let tmp = &mut make_rotY(args[1].parse::<f64>().unwrap());
                matrix_mult(&mut stack[last], tmp);
                stack[last] = tmp.clone();
            } else if args[0] == "z"{
                println!("\nRotating Frame about z at {} degrees", args[1]);
                let tmp = &mut make_rotZ(args[1].parse::<f64>().unwrap());
                matrix_mult(&mut stack[last], tmp);
                stack[last] = tmp.clone();
            } else {
                println!("Could not rotate due to no axis being specified");
            }
            println!("rotate {}", lines[cnt+1]);
            cnt += 2;
        } else if line == "circle"{
            let last : usize = stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing circle {} {} {} {}", args[0], args[1], args[2], args[3]);
            add_circle(&mut tmp, args[0], args[1], args[2], args[3], STEP);
            matrix_mult(&mut stack[last], &mut tmp);
            draw_lines(&mut tmp, &mut s, [0, 0, 0]);
            cnt += 2;
        } else if line == "hermite"{
            let last : usize = stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing hermite curve {} {} {} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            add_curve(&mut tmp, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], STEP, HERMITE);
            matrix_mult(&mut stack[last], &mut tmp);
            draw_lines(&mut tmp, &mut s, [0, 0, 0]);
            cnt += 2;
        } else if line == "bezier"{
            let last : usize= stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing bezier curve {} {} {} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            add_curve(&mut tmp, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], STEP, BEZIER);
            matrix_mult(&mut stack[last], &mut tmp);
            draw_lines(&mut tmp, &mut s, [0, 0, 0]);
            cnt += 2;
        } else if line == "box"{
            let last : usize= stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing box {} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5]);
            add_box(&mut tmp, args[0], args[1], args[2], args[3], args[4], args[5]);
            matrix_mult(&mut stack[last], &mut tmp);
            draw_polygons(&mut tmp, &mut s, [0, 0, 0], IS_BACKFACE_CULLED);
            cnt += 2;
        } else if line == "sphere"{
            let last : usize= stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing sphere {} {} {} {}", args[0], args[1], args[2], args[3]);
            add_sphere(&mut tmp, args[0], args[1], args[2], args[3], STEP2);
            matrix_mult(&mut stack[last], &mut tmp);
            draw_polygons(&mut tmp, &mut s, [0, 0, 0], IS_BACKFACE_CULLED);
            cnt += 2;
        } else if line == "torus"{
            let last : usize= stack.len()-1;
            let args = lines[cnt+1].split(" ").map(|l| l.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            println!("\nDrawing torus {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4]);
            add_torus(&mut tmp, args[0], args[1], args[2], args[3], args[4], STEP2); //
            matrix_mult(&mut stack[last], &mut tmp);
            draw_polygons(&mut tmp, &mut s, [0, 0, 0], IS_BACKFACE_CULLED);
            cnt += 2;
        } else if line == "display"{
            display(&mut s);
            cnt += 1;
        } else if line == "save" {
            save_ppm(&mut s, lines[cnt+1].to_string()); // will fix this eventually
            println!("save {:?}", lines[cnt+1]);
            cnt += 2;
        } else{
            println!("Unrecognized cmd {}. Moving on.", lines[cnt]);
            cnt += 1;
        }
    }
}
