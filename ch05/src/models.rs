pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub struct Color(pub i32, pub i32, pub i32);

pub struct Point(pub i32, pub i32, pub i32);

pub struct AlwaysEqual;

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn con_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
