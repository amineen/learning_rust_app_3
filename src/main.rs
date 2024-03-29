#![allow(unused)]

//import chrono crate
extern crate chrono;

use chrono::prelude::*;

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
    fn get_year_of_birth(&self) -> Option<i32> {
        let now = Utc::now();
        let current_year = now.year();
        self.age.map(|age| current_year - age as i32)
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

    let user1_yob = user.get_year_of_birth().unwrap_or(1990);
    let user2_yob = user2.get_year_of_birth().unwrap_or(1990);

    println!("{:?} was born in {}", user, user1_yob);
    println!("{:?} was born in {}", user2, user2_yob);

    let mut numbers = vec![1, 2, 3, 4, 5, 6];
    let numbers_iter = numbers.iter();

    let sum: i32 = numbers_iter.sum();
    numbers[1] = 22;
    println!("Sum of numbers: {}", sum);
    // println!("{:?}", numbers_iter);
    println!("original numbers: {:?}", numbers);
    numbers[1] = 22;
    numbers.push(7);
    println!("new numbers: {:?}", numbers);
}
