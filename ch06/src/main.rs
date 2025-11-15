use std::option;

use crate::models::{Coin, UsState, value_in_cents};

mod models;

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    // value_in_cents(coin);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // dice_roll(1000);

    // let config_max = Some(3u8);
    let config_max: Option<u8> = None;

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");

        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State querter form {state:?}");
        } else {
            count += 1;
            println!("{count}");
        }
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// 掷骰子，掷到1停2次，掷到6变为12，掷到指定总数停止
fn dice_roll(number: u32) {
    let mut count = 0u32;
    let mut dice = 0u32;
    loop {
        let mut point = rand::random_range(1..=6u32);

        match point {
            1 => {
                if dice <= number {
                    count = punish(count);
                }
            }
            6 => {
                point = double(point);
            }
            _ => (),
        };
        dice += point;
        count += 1;

        println!("{count:04}: {dice}");
        if dice >= number {
            break;
        }
    }
}

fn double(mut num: u32) -> u32 {
    num = num * 2;
    num
}

fn punish(mut count: u32) -> u32 {
    count += 2;
    count
}

fn roll(mut count: u32) -> u32 {
    count += 1;
    count
}
