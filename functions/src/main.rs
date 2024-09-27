fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value of x is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();
    let x = plus_one(x);
    print_labeled_measurement(5, 'h');
    println!("The value of x is: {x}");
}