fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn a<'a>(str1: &'a str, str2: &str) -> &'a str {
    println!("{str2}");
    str1
}

fn b<'a, 'b>(str1: &'a str, str2: &'b str) -> &'b str {
    println!("{str1}");
    str2
}

#[allow(dead_code)]
#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Student<'a> {
    fn new(name: &'a str) -> Self {
        Self { name: name, age: 0 }
    }
}

fn create_student<'a>(name: &'a str) -> Student<'a> {
    let steve = Student {
        name: name,
        age: 42,
    };

    steve
}

fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {result}");

    // println!("The longest string is {}", a(string2, string1.as_str()));

    // let name = String::from("Steve");
    // let steve = create_student(&name);
    // println!("{steve:?}");

    let str2 = String::from("hello");
    let c;
    {
        let str1 = "world";
        c = b(str1, str2.as_str());
    }
    println!("{c}");

    println!("{:?}", Student::new("Hello"));
}
