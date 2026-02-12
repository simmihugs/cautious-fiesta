#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    number: String,
}

fn main() {
    let mut user: User = User {
        name: "Peter Johnson".to_string(),
        age: 42,
        number: "kkkk".to_string(),
    };

    println!("{:?}", user);

    user.name = String::from("kkkkkk");

    println!("{:?}: age: {}", user, user.age);

    let mut user2 = User {
        name: "blablabla".to_string(),
        ..user.clone()
    };
    println!("{:?}", user2);

    user2.age = 33;
    user2.number = String::from("hello there");

    println!("{:?}", user);
    println!("{:?}", user2);
}
