fn main() {
    let arr = [0, 1, 2, 3, 4, 5];

    println!("Value at which index?");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("test");
    let index: usize = match buf.trim().parse() {
        Ok(i) => i,
        Err(_) => 42,
    };

    if index < arr.len() {
        println!("{}", arr[index])
    } else {
        let arr_str = arr
            .iter()
            .map(|x| x.to_string())
            .fold(String::new(), |mut arr, init| {
                arr = arr + " " + init.as_str();
                arr
            });
        panic!("index {index} is out of bound for array {arr_str}");
    }
}
