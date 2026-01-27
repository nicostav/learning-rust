use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!\n");

    loop {
        // Text first number
        println!("Insert the first number:");

        let mut first_number = String::new();

        // Read the first number of the calculation
        io::stdin().read_line(&mut first_number).expect("Failed to read line!");

        // Convert the string to a signed number
        let first_number: i32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Debug first number print
        println!("First number: {first_number}");

        // Text calculation sign
        println!("Enter the number of the calculation sign");
        println!("1: Addition");
        println!("2: Subtraction");
        println!("3: Multiplication");
        println!("4: Division");

        // Calculation variable
        let mut calculation_sign = String::new();
        // Read the wanted calculation sign
        match io::stdin().read_line(&mut calculation_sign) {
            Ok(_) => {
                
            }
            Err(error) => println!("Error reading line: {error}"),
        }

        // Convert string to unsigned number
        let calculation_sign: u32 = match calculation_sign.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Debug print calc sign
        println!("Calculation sign: {calculation_sign}");

        // Text second number
        println!("Insert the second number:");


        let mut second_number = String::new();

        // Read the first number of the calculation
        io::stdin().read_line(&mut second_number).expect("Failed to read line!");

        // Convert the string to a signed number
        let second_number: i32 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Debug print second number
        println!("Second number: {second_number}");

        // Check which calculation has to be performed ad do it
        if calculation_sign == 1 {
            let result: i32 = first_number + second_number;
            println!("Result of calculation {first_number} + {second_number} = {result}");
            break
        }
        if calculation_sign == 2 {
            let result: i32 = first_number - second_number;
            println!("Result of calculation {first_number} - {second_number} = {result}");
            break
        }
        if calculation_sign == 3 {
            let result: i32 = first_number * second_number;
            println!("Result of calculation {first_number} * {second_number} = {result}");
            break
        }
        if calculation_sign == 4 {
            let result: i32 = first_number / second_number;
            println!("Result of calculation {first_number} / {second_number} = {result}");
            break
        }
    }
}
