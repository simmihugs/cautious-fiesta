use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 42;
    let mut my_box = MyBox::new(x);
    println!("{my_box:?}");
    *my_box = 0;
    println!("{my_box:?}");
    *my_box = 42;
    println!("{my_box:?}");
    assert_eq!(42, x);
    assert_eq!(42, *my_box);

    let name = MyBox::new("Stefan");
    hello(&name);
}
