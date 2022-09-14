use regex::Regex;

fn make_operation(reg: Regex, mut expression: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }
    loop {
        // Aplicar operaciones
        let caps = reg.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_express = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result: i32 = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expression = expression.replace(cap_express, &result.to_string());
    }
    expression
}
fn main() {
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    // Traer datos del usuario
    println!("Por favor introduce una expresion:");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    expression = make_operation(re_div, expression, "/");
    expression = make_operation(re_mult, expression, "*");
    expression = make_operation(re_add, expression, "+");
    expression = make_operation(re_sub, expression, "-");
    // Mostrar el resultado
    println!("Resultado: {}", expression);
}
