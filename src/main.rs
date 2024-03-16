#![allow(unused)]

#[derive(Debug)]
struct User {
    user_id: i16,
    user_name: String,
}

impl User {
    fn set_user_id(&mut self, user_id: i16) {
        self.user_id = user_id;
    }
}

fn main() {
    let user = User {
        user_id: 1,
        user_name: String::from("Aaron Mineen"),
    };

    println!("User ID: {:?}", user);
}
