#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width: dbg!(width),
            height: dbg!(height),
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle::new(16, 16);
    dbg!(&rect);

    let rect2 = Rectangle::new(8, 15);

    println!(
        "does {rect2:?} completely fit into {rect:?}? -> {}",
        rect.can_hold(&rect2)
    );
}
