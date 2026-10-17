//Welcome to Ernst & Young (EY) Global Limited Interview System
use std::io;



fn main() {
    let mut input = String::new();

    println!("How many candidates are being interviewed?");
    io::stdin().read_line(&mut input);

    let number: u32 = match input.trim().parse() {
        Ok(value) => value,
        _ => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let mut candidates: Vec<(String, u32)> = Vec::new();

    for i in 0..number {
        println!("Enter the name of candidate {}:", i + 1);

        let mut name = String::new();
        io::stdin().read_line(&mut name);

        println!("Enter years of programming experience:");

        let mut experience = String::new();
        io::stdin().read_line(&mut experience);

        let years: u32 = match experience.trim().parse() {
            Ok(value) => value,
            _ => {
                println!("Please enter a valid number.");
                return;
            }
        };

        candidates.push((name.trim().to_string(), years));
    }

    let mut highest_name = String::new();
    let mut highest_years = 0;

    for (name, years) in &candidates {
        if *years > highest_years {
            highest_years = *years;
            highest_name = name.clone();
        }
    }

    println!("\n===== EY INTERVIEW RESULT =====");
    println!("Most experienced candidate: {}", highest_name);
    println!("Years of experience: {}", highest_years);
}




