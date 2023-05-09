use std::io;
use std::cmp::Ordering;
use chrono::prelude::*;

fn main() {
    println!("Enter your first name here: ");
    let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed to read input.");
    first_name = first_name.trim().to_string();

    println!("Enter your last name here: ");
    let mut last_name = String::new();
    io::stdin().read_line(&mut last_name).expect("Failed to read input.");
    last_name = last_name.trim().to_string();

    let birth_date = get_birth_date();

    let user_name = get_user_name(&first_name, &last_name, &birth_date);

    println!("Your username is: {}", user_name);
}

fn get_first_two_letters(name: &str) -> String {
    let first_two = &name[..2];
    first_two.to_lowercase()
}

fn get_first_four_letters(name: &str) -> String {
    let first_four = &name[..4];
    first_four.to_lowercase()
}

fn get_birth_date() -> String {
    let current_year = Utc::now().year();
    loop {
        println!("Please enter the day (DD) you were born: ");
        let mut day = String::new();
        io::stdin().read_line(&mut day).expect("Failed to read input.");
        let day: u32 = match day.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Oops! That was not a number. Try again...");
                continue;
            },
        };

        println!("Please enter the month (MM) you were born: ");
        let mut month = String::new();
        io::stdin().read_line(&mut month).expect("Failed to read input.");
        let month: u32 = match month.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Oops! That was not a number. Try again...");
                continue;
            },
        };

        println!("Please enter the year (YYYY) you were born: ");
        let mut year = String::new();
        io::stdin().read_line(&mut year).expect("Failed to read input.");
        let year: i32 = match year.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Oops! That was not a number. Try again...");
                continue;
            },
        };

        if day > 0 && day <= 31 && month > 0 && month <= 12 && year > 1800 && year <= current_year {
            let b_date = format!("{:02}-{:02}-{}", month, day, year);
            println!("Is your birthday (y/n?): {}", b_date);
            let mut loop_exit = String::new();
            io::stdin().read_line(&mut loop_exit).expect("Failed to read input.");
            let loop_exit = loop_exit.trim().to_lowercase();
            if loop_exit == "y" || loop_exit == "yes" {
                return b_date;
            }
        } else {
            println!("You entered invalid numbers. Try again...");
        }
    }
}

fn get_user_name(first_name: &str, last_name: &str, birth_date: &str) -> String {
    let user_name = get_first_two_letters(first_name) + &get_first_four_letters(last_name) + &birth_date[6..8];
    user_name
}

