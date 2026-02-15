trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    text: String,
}

fn summarize_it(t: &impl Summary) {
    println!("{}", t.summarize());
}

#[allow(dead_code)]
impl Article {
    fn new() -> Self {
        Article {
            text: String::new(),
        }
    }
    fn from(text: &str) -> Self {
        Article {
            text: String::from(text),
        }
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        let tmp = self.text.to_lowercase();
        if tmp.len() > 50 {
            tmp[..50].to_string() + "..."
        } else {
            tmp[..].to_string() + "..."
        }
    }
}

fn main() {
    let article = Article::from("Once upon a time there was a little shad. He was a bad bad bunny, and very insecure, which was a shame because he was a grifter as well.");

    summarize_it(&article);
}
