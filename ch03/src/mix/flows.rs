use std::{thread::sleep, time::Duration};

pub fn flows() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was true");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");

    // let mut count = 0u32;

    // loop {
    //     let millis = rand::random_range(50..2120);
    //     sleep(Duration::from_millis(millis));
    //     println!("{count}");
    //     count += 1;
    // }

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }

    for element in a {
        println!("{element}");
    }

    for number in (1..10).rev() {
        println!("LIFOFF");
    }
}
