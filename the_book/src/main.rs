#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<f64, f64> {
    fn wow(&self) -> Point<i32, i32> {
        Point {
            x: self.x.clone() as i32,
            y: self.y.clone() as i32,
        }
    }
}

fn main() {
    let point: Point<i32, f64> = Point { x: 4, y: 5.4 };
    println!("The point: {point:?}");

    println!("point: {:?}", (Point { x: 4.5, y: 8.9 }).wow());
}
