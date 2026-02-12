#[derive(Debug, Clone)]
struct RGB(u8, u8, u8);

impl RGB {
    fn red(&self) -> u8 {
        self.0
    }
    fn green(&self) -> u8 {
        self.1
    }
    fn blue(&self) -> u8 {
        self.2
    }
}

fn main() {
    let rgb: RGB = RGB(255, 255, 255);
    println!("{:?}", rgb);
    println!(
        "red: {0}\tgreen: {1}\tblue: {2}",
        rgb.red(),
        rgb.green(),
        rgb.blue()
    );
}
