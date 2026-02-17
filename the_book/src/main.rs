#[allow(dead_code)]
struct Haus {
    a: i32,
}

impl Drop for Haus {
    fn drop(&mut self) {
        println!("So long and thanks for all the fish!");
    }
}

fn main() {
    {
        let _haus = Haus { a: 42 };
    }
    let haus = Haus { a: 42 };
    drop(haus);
}
