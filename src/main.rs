fn main() {
    // slices let you reference a contiguous sequence of elements in a collection.

    let s = String::from("hello, world!");
    let slice = get_slice(&s);
    slice.chars().for_each(|x| {
        println!("{}", x);
    });

    println!("");
    first_word(&s).chars().for_each(|x| {
        println!("{}", x);
    });
}

fn get_slice(s: &String) -> &str {
    &s[4..]
}

fn first_word(s: &String) -> &str {
    let byts = s.as_bytes();
    for (i, &b) in byts.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
