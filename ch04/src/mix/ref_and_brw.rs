pub fn ref_and_brw() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");
}

fn calculate_length(s: &str) -> usize {
    s.len()
}
