fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exit = &v[100];
    // println!("{does_not_exit}");

    let does_not_exit2 = v.get(100);
    println!("{does_not_exit2:?}");
}
