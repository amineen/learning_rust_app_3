#![allow(unused)]

#[derive(Debug)]
struct User {
    user_id: i16,
    user_name: String,
    age: Option<i8>,
    email: Option<String>,
}

impl User {
    fn set_age(&mut self, age: i8) {
        self.age = Some(age);
    }

    fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }
}

fn main() {
    let mut user = User {
        user_id: 1,
        user_name: String::from("Aaron Mineen"),
        age: None,
        email: None,
    };

    user.set_age(25);

    let user_age: i8 = user.age.unwrap_or(0);

    user.set_email(String::from("mineen@gmail.com"));

    println!("{:?} User Age:{}", user, user_age);
}
