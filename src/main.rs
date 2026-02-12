fn dangling() -> String {
    let s = String::from("hello");

    s
}

fn main() {
    let _reference = dangling();
}
