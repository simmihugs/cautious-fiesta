fn main() {
    // let guess: i32 = " 42 ".trim().parse().expect("could not parse");
    // println!("guess: {guess}");

    // let tup: (&str, &str, i32) = ("hello", "world!", 42);
    // println!("{0}", tup.0);

    // let (x, y, _) = tup;
    // println!("x: {x}\ty: {y}");

    // let arr = [1, 2, 3];
    // arr.iter().for_each(|x| println!("{}", x));

    // ['a', 'b', 'c']
    //     .iter()

    //     .for_each(|(i, y)| print!("{y} "));
    // println!("");

    let b = [0; 13];
    b.iter()
        .enumerate()
        .for_each(|(i, y)| println!("{i:3}: {y:3}"));
}
