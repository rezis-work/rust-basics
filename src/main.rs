fn main() {
    let success: Result<i32, String> = Result::Ok(42);
    let error: Result<i32, String> = Result::Err("Something went wrong".to_string());
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

