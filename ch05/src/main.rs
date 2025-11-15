use crate::models::{Color, Point, Rectangle, User};

mod models;

fn main() {
    let mut user1 = models::User {
        active: true,
        username: String::from("smoeusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("choi@example.com"),
        username: String::from("choi"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("Can rect1 hold rect2: {}", rect1.con_hold(&rect2));
    println!("Can rect1 hold rect2: {}", rect1.con_hold(&rect3));
}
