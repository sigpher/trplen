pub(crate) fn guessing_game() {
    println!("Guessing Game\nPlease input your guess:");
    let secret_number = rand::random_range(1..100u8);
    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Small"),
            std::cmp::Ordering::Greater => println!("Too Big"),
            std::cmp::Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }
}
