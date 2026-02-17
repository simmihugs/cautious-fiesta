fn main() {
    let mut vec: Vec<i32> = (0..10).collect();
    for i in &mut vec {
        *i = 42;
    }
    println!("{vec:#?}");

    println!("{}", vec.iter().sum::<i32>());

    println!("{vec:#?}");

    let vec: Vec<i32> = (0..10).collect();
    println!(
        "{:#?}",
        vec.iter().filter(|x| **x % 2 == 0).collect::<Vec<&i32>>()
    );

    let vec: Vec<i32> = vec
        .iter()
        .filter(|x| **x % 2 == 0)
        .map(|&x| x)
        .collect::<Vec<i32>>();

    println!("{:#?}", vec);
}
