fn some_stuff(s: &str) -> &str {
    println!("wow: {s}");
    &s[..]
}

fn main() {
    // string literals

    let s = "hello, world!";
    let _s = some_stuff(s);
    let ss = String::from("asdfbasdf asdfsd");
    let _ss = some_stuff(&ss);
}
