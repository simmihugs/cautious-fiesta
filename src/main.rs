#[derive(Debug, Clone)]
struct RGB(u8, u8, u8);

fn main() {
    let rgb: RGB = RGB(255, 255, 255);
    println!("{:?}", rgb);
    println!("red: {0}\tgreen: {1}\tblue: {2}", rgb.0, rgb.1, rgb.2);
}
