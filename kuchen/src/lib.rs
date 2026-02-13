#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Wait..");
        }

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {
            super::hosting::add_to_waitlist();
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}

pub fn eat2() {
    front_of_house::serving::take_order();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
