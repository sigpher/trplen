use std::{ error::Error, fs::{ self, File }, io::{ self, ErrorKind::{ self, NotFound }, Read } };

fn main() -> Result<(), Box<dyn Error>> {
    let mut greeting_file_result = File::open("hello.txt");

    // let mut greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) =>
    //         match error.kind() {
    //             NotFound =>
    //                 match File::options().append(true).create(true).open("hello.txt") {
    //                     Ok(fc) => fc,
    //                     Err(error) => panic!("Problem creating the file :{error:?}"),
    //                 }
    //             _ => panic!("Problem opening the file: {error:?}"),
    //         }
    // };

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem createing file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn read_username_form_file() -> Result<String, io::Error> {
    // let mut username = String::new();
    // let mut username_file = File::open("hello.txt")?.read_to_string(&mut username);
    // Ok(username)
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
