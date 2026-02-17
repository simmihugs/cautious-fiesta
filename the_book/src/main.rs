struct Silly {
    // a: Option<i32>,
    // b: Option<i32>,
    // c: Option<i32>,
    content: Vec<i32>,
}

struct SillyIter<'a> {
    silly: &'a Silly,
    state: usize,
}

impl Silly {
    fn iter(&self) -> SillyIter<'_> {
        SillyIter {
            silly: self,
            state: 0,
        }
    }
}

impl<'a> Iterator for SillyIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        // let result = match self.state {
        //     0 => self.silly.a.as_ref(),
        //     1 => self.silly.b.as_ref(),
        //     2 => self.silly.c.as_ref(),
        //     _ => None,
        // };

        let result = if self.state < self.silly.content.len() {
            Some(&self.silly.content[self.state])
        } else {
            None
        };

        self.state += 1;
        result
    }
}

fn main() {
    let silly = Silly {
        // a: Some(42),
        // b: Some(42),
        // c: Some(42),
        content: (0..10).collect(),
    };

    silly.iter().for_each(|x| {
        println!("{x:?}");
    });
    // for i in silly {
    //     println!("{i}");
    // }
}
