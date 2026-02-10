fn main() {
    (1..10).rev().for_each(|a| {
        println!("{a}");
    });

    (0..5).for_each(|x| {
        println!("{x}");
    });
}
