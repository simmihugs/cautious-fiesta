#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum Shape {
    Circle {
        center: Point,
        radius: u32,
    },
    Square {
        upper_left: Point,
        width: u32,
        height: u32,
    },
}

use Shape::*;

impl Shape {
    fn center(&self) -> &Point {
        match self {
            Shape::Circle { center, .. } => center,
            _ => panic!("I cant even: {self:?}"),
        }
    }
}

fn main() {
    let circle = Shape::Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 13,
    };

    let square = Shape::Square {
        upper_left: Point { x: 0.0, y: 0.0 },
        width: 12,
        height: 12,
    };

    println!("circle: {circle:?}");
    println!("{}", circle.center().x);

    // causes panic!
    //println!("{}", square.center().x);

    let circle2 = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 13,
    };
    println!("{circle2:?}");
}
