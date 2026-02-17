fn main() {
    let mut x: Box<i32> = Box::new(5);
    assert_eq!(*x, 5);
    *x += 42;
    assert_eq!(*x, 47);

    println!("x = {x}");
}
