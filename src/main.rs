pub mod generator;

use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Error: Less than 3 arguments.");
        write_usage(&args[0]);
        return;
    }

    let limit: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(e) => {
            println!("Invalid limit: {e}");
            write_usage(&args[0]);
            return;
        }
    };

    let filename: &str = &args[2];
    let lang_type: &str = &args[3];

    println!("Generating all {limit} cases...");
    let code: String = match generator::code::generate(lang_type, limit) {
        Ok(s) => s,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!("Writing all {limit} cases...");
    match write_to_file(&code, filename) {
        Ok(_) => println!("Write successful!"),
        Err(e) => println!("Write failed. {e}"),
    };
}

fn write_usage(exec_name: &str) {
    println!("Usage: {exec_name} <LIMIT> <FILENAME> <LANGUAGE>");
}

fn write_to_file(content: &str, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", content)?;
    Ok(())
}
