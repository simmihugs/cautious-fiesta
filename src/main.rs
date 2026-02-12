fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;
    println!("{r3}");

    let r4 = &s;
    let r5 = &s;
    println!("{r4} {r5}");
}
