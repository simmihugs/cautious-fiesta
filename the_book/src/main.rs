fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    let res = add(40, 2);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kuchen() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
