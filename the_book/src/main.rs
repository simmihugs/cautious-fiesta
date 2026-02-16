struct Silly {
    a: Option<i32>,
    b: Option<i32>,
    c: Option<i32>,
}

impl Iterator for Silly {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.a.take() {
            Some(v)
        } else if let Some(v) = self.b.take() {
            Some(v)
        } else if let Some(v) = self.c.take() {
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    let silly = Silly {
        a: Some(42),
        b: Some(42),
        c: Some(42),
    };

    // silly.iter().for_each(|x| {
    //     println!("{x:?}");
    // })
    for i in silly {
        println!("{i}");
    }
}
