struct Silly {
    a: Option<i32>,
    b: Option<i32>,
    c: Option<i32>,
}

impl Iterator for Silly {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.a {
            Some(v) => {
                self.a = None;
                Some(v)
            }
            None => match self.b {
                Some(v) => {
                    self.b = None;
                    Some(v)
                }
                None => match self.c {
                    Some(v) => {
                        self.c = None;
                        Some(v)
                    }
                    None => None,
                },
            },
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
