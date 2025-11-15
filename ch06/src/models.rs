pub enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from '{state:?}'!");
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
}
