use std::{env, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("bf: missing arguments");
        process::exit(1);
    } else {
        let inputfile = Path::new(&args[1]);

        if inputfile.exists() {
            match fs::read_to_string(inputfile) {
                Ok(_) => {}
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
