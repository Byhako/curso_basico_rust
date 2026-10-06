use regex::Regex;
use std::io::{self, Write};

fn main() {
    println!("\nBienvenido a tu calculadora.");

    // Regex
    let re_mul = Regex::new(r"(\d+(?:\.\d+)?)\s*\*\s*(\d+(?:\.\d+)?)").unwrap();
    let re_div = Regex::new(r"(\d+(?:\.\d+)?)\s*/\s*(\d+(?:\.\d+)?)").unwrap();
    let re_add = Regex::new(r"(\d+(?:\.\d+)?)\s*\+\s*(\d+(?:\.\d+)?)").unwrap();
    let re_sub = Regex::new(r"(\d+(?:\.\d+)?)\s*-\s*(\d+(?:\.\d+)?)").unwrap();

    let is_number = Regex::new(r"^-?\d+(?:\.\d+)?$").unwrap();

    // Datos del usuario
    print!("\nTu operación > ");
    io::stdout().flush().unwrap();

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();

    let mut result: f64 = 0.0;


    loop {
        // Multiplicacion
        if let Some(caps) = re_mul.captures(&operation) {
            let expression = &caps[0];
            let num1 = caps[1].parse::<f64>().unwrap();
            let num2 = caps[2].parse::<f64>().unwrap();

            result = num1 * num2;
            operation = operation.replace(expression, &result.to_string());
        } else
        // Division
        if let Some(caps) = re_div.captures(&operation) {
            let expression = &caps[0].to_string();
            let num1 = caps[1].parse::<f64>().unwrap();
            let num2 = caps[2].parse::<f64>().unwrap();

            if num2 == 0.0 {
                println!("\n❌ Error matemático: No se puede dividir por cero.");
                break;
            }

            result = num1 / num2;
            operation = operation.replace(expression, &result.to_string());
        } else
        // Suma
        if let Some(caps) = re_add.captures(&operation) {
            let expression = &caps[0].to_string();
            let num1 = caps[1].parse::<f64>().unwrap();
            let num2 = caps[2].parse::<f64>().unwrap();

            result = num1 + num2;

            operation = operation.replace(expression, &result.to_string());
        } else
        // Resta
        if let Some(caps) = re_sub.captures(&operation) {
            let expression = &caps[0].to_string();
            let num1 = caps[1].parse::<f64>().unwrap();
            let num2 = caps[2].parse::<f64>().unwrap();

            result = num1 - num2;
            operation = operation.replace(expression, &result.to_string());
        } else {
            if !is_number.is_match(&operation.trim()) {
                println!("\n❌ Operación no reconocida.");
                break;
            }

            println!("\nResultado: {}", result);
            break;
        }
    }

}
