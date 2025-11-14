pub fn parameters() {
    another_function(10);
    print_labeled_measurement(10, "km/h".into());
}

fn another_function(x: u32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit: String) {
    let output = format!("{value} {unit}");
    println!("{output}");
}
