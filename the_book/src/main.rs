fn main() {
    let mut vec: Vec<i32> = (0..10).collect();
    for i in &mut vec {
        *i = 42;
    }
    println!("{vec:#?}");

    println!("{}", vec.iter().sum::<i32>());

    println!("{vec:#?}");
}
