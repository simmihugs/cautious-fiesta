const HELLO_WORLD: &str = "hello, world!";
const NUMBER: i32 = 42 * 1;

fn main() {
    println!("{}, {NUMBER}", HELLO_WORLD);

    let x = 42;
    println!("x: {x}");
    let x = x + 42;
    println!("x: {x}");
    {
        let x = 15;
        println!("x: {x}");
        let x = "   ".len();
        println!("x: {x}");
    }
    println!("x: {x}");
}
