#[derive(Debug, Clone)]
struct Element;

trait Strategy {
    fn fun(&self);
}

impl Strategy for Element {
    fn fun(&self) {
        println!("{}", "hello, world!");
    }
}

fn main() {
    let element = Element;

    element.fun();
}
