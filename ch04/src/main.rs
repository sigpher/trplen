mod mix;
fn main() {
    mix::ownership();
    mix::ref_and_brw();
    let first = mix::first_word("hello world!");
    println!("{first}");
}
