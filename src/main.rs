fn main() {
    let add_result = add_both(1.1, 2.2);
    let subtract_result = subtract_both(1.1, 2.2);
    let multiply_result = multiply_both(1.1, 2.2);
    let divide_result = divide_both(1.1, 2.2);

    println!("add_result is {}", add_result);
    println!("subtract_result is {}", subtract_result);
    println!("multiply_result is {}", multiply_result);
    println!("divide_result is {}", divide_result);

    let general_result = format!("add: {}, subtract: {}, multiply: {}, divide: {}", add_result, subtract_result, multiply_result, divide_result);

    println!("general_result is {}", general_result);
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return  x * y;
}

fn add_both(x: f64, y: f64) -> f64 {
    return x + y;
}

fn subtract_both(x: f64, y: f64) -> f64 {
    return x - y;
}

fn divide_both(x: f64, y: f64) -> f64 {
    return x / y;
}
