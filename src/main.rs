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

    let mut vec: Vec<i32> = (0..10).collect();
    vec.iter().enumerate().for_each(|(i, &x)| {
        println!("{i}: {x}");
    });
    println!("");
    vec.clear();
    get_vec_slice(&vec).iter().enumerate().for_each(|(i, &x)| {
        println!("{i}: {x}");
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

fn get_vec_slice(v: &Vec<i32>) -> &[i32] {
    &v[..3]
}
