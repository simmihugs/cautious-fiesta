fn main() {
    let mut x = 5;
    let y = x;
    println!("x = {x}");
    println!("y = {y}");

    x = 42;

    println!("x = {x}");
    println!("y = {y}");

    let mut x = String::from("abc");
    let y = x.clone();

    println!("x = {x}");
    println!("y = {y}");

    x = "cde".to_string();

    println!("x = {x}");
    println!("y = {y}");
}
