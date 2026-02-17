#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

fn main() {
    let start = Nil;
    let next = Cons(1, Box::new(start));
    let next_next = Cons(2, Box::new(next));
    println!("{next_next:?}");
}
