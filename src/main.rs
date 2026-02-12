#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let mut user: User = User {
        name: "Peter Johnson".to_string(),
        age: 42,
    };

    println!("{:?}", user);

    user.name = String::from("kkkkkk");

    println!("{:?}: age: {}", user, user.age);
}
