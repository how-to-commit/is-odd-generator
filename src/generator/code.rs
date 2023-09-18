use super::error::GeneratorError;

pub fn generate(lang_type: &str, num_reps: u32) -> Result<String, GeneratorError> {
    if lang_type == "python" {
        let code: String = generate_python(num_reps);
        return Ok(code);
    } else if lang_type == "javascript" {
        let code: String = generate_javascript(num_reps);
        return Ok(code);
    }
    return Err(GeneratorError {
        error_msg: format!("Could not recognise the language {lang_type}."),
    });
}

fn generate_python(num_reps: u32) -> String {
    let mut code: String = "".to_string();
    code.push_str("def is_odd(n): \n    if n == 1:\n        return True\n");
    for num in 2..num_reps + 1 {
        let is_odd: &str = if num % 2 == 0 { "False" } else { "True" };
        code.push_str(&format!("    elif n == {num}:\n        return {is_odd}\n"));
    }
    return code;
}

fn generate_javascript(num_reps: u32) -> String {
    let mut code: String = "".to_string();
    code.push_str("function is_odd(n){ \n    if (n === 1) {\n        return true\n    }\n");
    for num in 2..num_reps + 1 {
        let is_odd: &str = if num % 2 == 0 { "false" } else { "true" };
        code.push_str(&format!(
            "    else if (n === {num}) {{\n        return {is_odd}\n    }}\n"
        ));
    }
    code.push_str("}");
    return code;
}
