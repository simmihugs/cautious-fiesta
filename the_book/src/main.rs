use std::thread;

fn haus() {
    println!("hello");
}

fn france(s: &str) {
    println!("{s:?}");
}

fn main() {
    let vec: Vec<i32> = (0..10).collect();
    println!("vector: {vec:?}");

    thread::spawn(move || println!("vector: {vec:?}"))
        .join()
        .unwrap();

    //println!("vector: {vec:?}");

    thread::spawn(haus).join().unwrap();

    let s = String::from("i");
    thread::spawn(move || france(&s)).join().unwrap();

    main2();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main2() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| (r.width % 10).to_string().len());
    println!("{list:#?}");
}
