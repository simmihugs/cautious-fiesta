fn main() {
    let v: Vec<i32> = (0..10).collect();
    v.iter().for_each(|x| {
        println!("{x}");
    });

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
}
