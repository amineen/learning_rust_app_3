#![allow(unused)]

#[derive(Debug)]
struct User {
    user_id: i16,
    user_name: String,
    age: Option<i8>,
    email: Option<String>,
}

impl User {
    fn new(id: i16, name: &str) -> User {
        User {
            user_id: id,
            user_name: name.to_string(),
            age: None,
            email: None,
        }
    }

    fn age(&mut self, age: i8) -> &mut User {
        self.age = Some(age);
        self
    }

    fn email(&mut self, email: &str) -> &mut User {
        self.email = Some(email.to_string());
        self
    }
}

struct UserBuilder {
    user_id: i16,
    user_name: String,
    age: Option<i8>,
    email: Option<String>,
}

impl UserBuilder {
    fn new(id: i16, name: &str) -> UserBuilder {
        UserBuilder {
            user_id: id,
            user_name: name.to_string(),
            age: None,
            email: None,
        }
    }

    fn age(&mut self, age: i8) -> &mut UserBuilder {
        self.age = Some(age);
        self
    }

    fn email(&mut self, email: &str) -> &mut UserBuilder {
        self.email = Some(email.to_string());
        self
    }

    fn build(&self) -> User {
        User {
            user_id: self.user_id,
            user_name: self.user_name.clone(),
            age: self.age,
            email: self.email.clone(),
        }
    }
}

fn main() {
    let mut user = User::new(1, "Aaron Mineen");
    user.age(25).email("mineen@gmail.com");

    let mut user2 = UserBuilder::new(2, "John Doe")
        .age(30)
        .email("johndoe@gmail.com")
        .build();

    println!("{:?}", user);
    println!("{:?}", user2);
}
