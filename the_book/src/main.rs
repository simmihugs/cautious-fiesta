fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    //let vv = v.get(0);

    v.push(6);

    println!("first: {:?}", v.get(0));
    println!("last: {:?}", v.last());
}
