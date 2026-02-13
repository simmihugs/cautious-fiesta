use kuchen;

fn main() {
    let x: u64 = 42;
    let y: u64 = 0;
    println!("Add {x} and {y} = {}", kuchen::add(x, y));
}
