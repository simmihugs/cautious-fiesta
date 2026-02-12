fn dangling() -> &String {
    let mut s = String::from("hello");

    &s
}

fn main() {
    let reference = dangling();
}
