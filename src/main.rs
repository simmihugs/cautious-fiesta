fn main() {
    let s = "hello".to_string();

    does_not_take_ownership(&s);
    println!("{s}");
}

fn does_not_take_ownership(s: &String) {
    println!("{s}");
}
