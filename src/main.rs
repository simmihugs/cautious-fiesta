fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let config_max2: Option<i32> = None;
    if let Some(max) = config_max2 {
        println!("The maximum is configured to be {max}");
    }
}
