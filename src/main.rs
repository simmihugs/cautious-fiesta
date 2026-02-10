fn main() {
    let mut s = String::from("banane");

    println!("{}", s);

    update_string(&mut s);

    println!("{}", s);
}

fn update_string(s: &mut String) {
    s.push_str("neis");
}
