fn value_it(coin: &Option<i32>) -> Option<String> {
    let Some(value) = coin else {
        return None;
    };

    if *value > 42 {
        Some(format!("value: {value} is high!"))
    } else {
        Some(format!("value: {value} is not high!"))
    }
}
fn main() {
    let coin = Some(32);
    let value = value_it(&coin);
    println!("{value:?}");
}
