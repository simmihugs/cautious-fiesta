#[derive(Debug)]
enum Ipaddr {
    #[allow(dead_code)]
    V4(String),
    #[allow(dead_code)]
    V6(String),
}

fn main() {
    let v4 = Ipaddr::V4("127.0.0.1".to_string());
    let v6 = Ipaddr::V6("::1".to_string());

    println!("v4: {v4:?}\tv6: {v6:?}");
}
