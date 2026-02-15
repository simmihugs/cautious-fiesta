#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let point: Point<i32, f64> = Point { x: 4, y: 5.4 };
    println!("The point: {point:?}");
}
