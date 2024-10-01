use std::{env, fs, path::Path, process};

mod bf;
use bf::Brainfuck;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("bf: missing arguments");
        process::exit(1);
    } else {
        let inputfile = Path::new(&args[1]);

        if inputfile.exists() {
            match fs::read_to_string(inputfile) {
                Ok(content) => {
                    let brainfuck = Brainfuck::new(content, args.get(2));
                    if let Err(e) = brainfuck.exec() {
                        println!("{e}");
                    }
                }
                Err(e) => {
                    println!("{e}");
                    process::exit(1);
                }
            }
        } else {
            println!("bf: invalid arguments");
            process::exit(1);
        }
    }
}
