use std::io;     
use std::process;

fn main() {
    println!("Temperature Converter! If you'd like to exit, type 'exit'");

    'input: loop {
        println!("Enter temp and unit: (ex: 100C)");
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let temp_tup = handle_input(input);
        
        
    }
}

// fn f_to_c(temp_in_f: f32) -> f32 {
//     return 0.0;
// }

fn handle_input(input: String) -> (f32, char) {
    if input.trim().to_lowercase() == "exit" || input.trim().to_lowercase() == "bye" {
        println!("Goodbye!");
        process::exit(0);
    }

    let unit = input.trim().chars().last().unwrap().to_ascii_uppercase();

    let temp_string: String = input.chars().filter(|x| x.to_ascii_uppercase().ne(&unit)).collect::<String>();
    let temp: f32 = temp_string.trim().parse().unwrap();
    return (temp, unit);
}
