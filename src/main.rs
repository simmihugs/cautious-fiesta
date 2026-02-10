fn main() {
    let mut count = 1;
    'counting_up: loop {
        let mut loop_var = 10;

        if count < 3 {
            println!("loop: {count}");
        }

        loop {
            if count == 3 {
                break 'counting_up;
            }

            if loop_var == 0 {
                break;
            }
            println!("\tloop_2: {loop_var}");
            loop_var -= 1;
        }
        count += 1;
    }
}
