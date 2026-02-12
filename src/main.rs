fn main() {
    // slices let you reference a contiguous sequence of elements in a collection.

    let s = String::from("hello, world!");
    let slice = get_slice(&s);
    slice.chars().for_each(|x| {
        println!("{}", x);
    })
}

fn get_slice(s: &String) -> &str {
    &s[..4]
}
