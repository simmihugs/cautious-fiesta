fn main() {
    let mut arr: Vec<i32> = vec![];
    for _ in 0..10 {
        arr.push(rand::random_range(10..30));
    }

    arr.iter().for_each(|x| {
        println!("{x}");
    });
    println!("");

    arr = arr
        .into_iter()
        .filter(|x| *x > 10 && *x % 2 != 0)
        .collect::<Vec<i32>>();

    arr.iter().for_each(|x| {
        println!("{x}");
    });
}
