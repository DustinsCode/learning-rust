fn main() {
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let y = plus_one(x);

    println!("The value of y is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5 //this expression's value is the return value
}

fn plus_one(x: i32) -> i32 {
    // if we want to add a semicolon, have to also add `return`
    x + 1
}