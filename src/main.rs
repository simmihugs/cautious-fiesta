fn main() {
    let s = "hello".to_string();

    takes_ownership(s);
    //println!("{s}");

    let x = 5;
    makes_copy(x);
    println!("{x}");
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn makes_copy(x: i32) {
    println!("{x}");
}
