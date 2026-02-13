use kuchen::blechkuchen as kuchen;

fn main() {
    kuchen::gelee();
    kuchen::torte::sahne();

    println!("Manfred Meisel");

    let v: Vec<i32> = (0..10).collect();
    v.iter().for_each(|x| {
        println!("{x}");
    });
}
