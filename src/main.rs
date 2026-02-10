fn main() {
    let mut count = 0;

    while count != 10 {
        println!("{count}");
        count += 1;

        if count % 2 == 0 {
            break;
        }
    }
}
