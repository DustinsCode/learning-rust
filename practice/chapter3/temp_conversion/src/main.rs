use std::io;     
use std::process;

fn main() {
    println!("Temperature Converter! If you'd like to exit, type 'exit'");

    loop {
        println!("Enter temp and unit: (ex: 100C)");
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (temp, unit) = handle_input(input);
        
        if unit == 'F' {
            let temp_in_c = f_to_c(temp);
            println!("{temp}F is {temp_in_c}C");
        } else if unit == 'C' {
            let temp_in_f = c_to_f(temp);
            println!("{temp}C is {temp_in_f}F");
        } else {
            println!("Invalid unit, please only enter C or F");
        }
    }
}

fn f_to_c(temp_in_f: f32) -> f32 {
    return (temp_in_f - 32.0) * 5.0 / 9.0;
}

fn c_to_f(temp_in_c: f32) -> f32 {
    return (temp_in_c * 1.8) + 32.0;
}

fn handle_input(input: String) -> (f32, char) {
    if input.trim().to_lowercase() == "exit" || input.trim().to_lowercase() == "bye" {
        println!("Goodbye!");
        process::exit(0);
    }

    let unit = input.trim().chars().last().unwrap().to_ascii_uppercase();

    let temp_string: String = input.chars().filter(|x| x.to_ascii_uppercase().ne(&unit)).collect::<String>();
    let temp: f32 = temp_string.trim().parse().unwrap_or_default();
    return (temp, unit);
}
