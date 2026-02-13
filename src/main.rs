fn add_to(value: i32, o: &mut Option<i32>) {
    match o {
        None => (),
        Some(v) => *o = Some(*v + value),
    }
}

fn main() {
    let mut o = Some(32);
    println!("value: {o:?}");
    add_to(10, &mut o);
    println!("value: {o:?}");

    let mut o2 = None;
    println!("value: {o2:?}");
    add_to(10, &mut o2);
    println!("value: {o2:?}");
}
