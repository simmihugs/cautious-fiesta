#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    //let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        //sort_operations.push(value.clone());
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}");
    println!("num_sort_operations: {num_sort_operations}");
}
