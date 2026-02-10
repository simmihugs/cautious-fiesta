fn fibonacci(nth: i32) -> i32 {
    if nth == 0 {
        0
    } else if nth == 1 {
        1
    } else {
        fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}

fn main() {
    println!("{}", fibonacci(5));
}
