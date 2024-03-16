#![allow(unused)]

#[derive(Debug)]
struct User {
    user_id: i16,
    user_name: String,
    age: Option<i8>,
}

impl User {
    fn set_age(&mut self, age: i8) {
        self.age = Some(age);
    }
}

fn main() {
    let mut user = User {
        user_id: 1,
        user_name: String::from("Aaron Mineen"),
        age: None,
    };

    // user.set_age(25);

    let user_age: i8 = user.age.unwrap_or(0);

    println!("User:{:?} User Age:{}", user, user_age);
}
