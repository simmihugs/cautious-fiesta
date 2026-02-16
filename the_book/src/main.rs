static CLOSURE: fn(i32) -> i32 = |x| x + 2;

fn main() {
    println!("{}", CLOSURE(42));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(42, CLOSURE(40));
    }
}
