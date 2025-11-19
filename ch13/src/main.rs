use std::{fmt::Display, thread};

mod closure;
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };

    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );

    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );

    pub fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    let add_one_v2 = |x| x + 1;

    add_one_v2(1);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list_2 = vec![1, 2, 3];
    println!("Before definging closure:{list_2:?}");
    let mut borrows_mutably = || list_2.push(7);
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure:{list_2:?}");

    let list_3 = vec![1, 2, 3];
    println!("Before defining closure: {list_3:?}");
    thread::spawn(move || println!("From tread: {list_3:?}"))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    for rect in list {
        println!("{rect}");
    }
}

enum MyOption<T> {
    Some(T),
    None,
}

use MyOption::*;

impl<T> MyOption<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect:{}, {}", self.width, self.height)
    }
}
