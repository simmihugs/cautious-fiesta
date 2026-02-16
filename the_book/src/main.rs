static CLOSURE: fn(i32) -> i32 = |x| x + 2;

fn main() {
    println!("{}", CLOSURE(42));

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = vec![];
    println!("{list:?}");
    let mut hurra = || list.push(42);
    for _ in 0..10 {
        hurra();
    }
    println!("{list:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(42, CLOSURE(40));
    }
}
