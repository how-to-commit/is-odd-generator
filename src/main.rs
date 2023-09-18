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
    let code: String = match generate_code(lang_type, limit) {
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

fn generate_code(lang_type: &str, num_reps: u32) -> Result<String, String> {
    let mut code: String = "".to_string();

    if lang_type == "python" {
        code.push_str("def is_odd(n):\n");
        for num in 1..num_reps + 1 {
            let is_odd: &str = if num % 2 == 0 { "False" } else { "True" };
            code.push_str(&format!("    if n == {num}:\n        return {is_odd}\n")[..]);
        }
        return Ok(code);
    }
    // make an actual error type pls
    return Err(format!("Could not recognise the language {lang_type}."));
}

fn write_to_file(content: &str, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", content)?;
    Ok(())
}
