fn find_largest<T: std::cmp::PartialOrd<T>>(vec: &[T]) -> &T {
    let mut largest = &vec[0];
    for v in vec {
        if *v > *largest {
            largest = v;
        }
    }

    largest
}

fn main() {
    let vec: Vec<i32> = (0..10).collect();
    let largest = find_largest(&vec);
    println!("for vec {vec:?} the largest element is: {largest}\n\n");

    let vec: Vec<char> = ('a'..='z').collect();
    let largest = find_largest(&vec);
    println!("for vec {vec:?} the largest element is: {largest}\n\n");
}
