fn overtime_check(hours: f64, pay: f64) -> f64 {
    if hours > 40.0 {
        return (40.0 * pay) + ((hours - 40.0) * (pay * 1.5));
    } else {
        return hours * pay;
    }
}

fn main() {
    // 1. Conditional Basics

    // 1-a. prompt the user for a day of the week, print out whether the day is Monday or not
    println!("Day of the week?");
    let mut day = String::new();
    std::io::stdin().read_line(&mut day).expect("Failed to read input.");
    day = day.trim().to_lowercase();

    if day == "monday" {
        println!("Yes, it's Monday.");
    } else {
        println!("It's not Monday.");
    }

    // 1-b. prompt the user for a day of the week, print out whether the day is a weekday or a weekend
    println!("What day of the week is it?");
    let mut day = String::new();
    std::io::stdin().read_line(&mut day).expect("Failed to read input.");
    day = day.trim().to_lowercase();

    if day == "saturday" || day == "sunday" {
        println!("Weekend baby!");
    } else {
        println!("Back to the grind...");
    }

    // 1-c. create variables and make up values for
    // 1c-I. the number of hours worked in one week
    let hours_worked = 52.0;

    // 1c-II. the hourly rate
    let hourly_rate = 16.50;

    // 1c-III. how much the week's paycheck will be
    let paycheck = overtime_check(hours_worked, hourly_rate);
    println!("Paycheck: ${}", paycheck);
    println!("Hold your horses!\n\nYou have to pay Uncle Sam!");
    println!("Paycheck AFTER taxes: ${}", paycheck * 0.7);

    // 2. Loop Basics

    // 2a. While

    // 2a-I. Create an integer variable i with a value of 5.
    let mut i = 5;

    // 2a-II. Create a while loop that runs so long as i is less than or equal to 15
    while i <= 15 {
        println!("{}", i);
        i += 1;
    }

    // 2a-IV. Create a while loop that will count by 2's starting with 0 and ending at 100.
    while i <= 100 {
        println!("{}\n", i);
        i += 2;
    }

    // 2a-V. Alter your loop to count backwards by 5's from 100 to -10
    i = 100;
    while i >= -10 {
        println!("{}\n", i);
        i -= 5;
    }

    // 2a-VI. Create a while loop that starts at 2 and displays the number squared on each line
    // while the number is less than 1,000,000.
    i = 2;
    while i < 1_000_000 {
        println!("{}", i);
        i *= i;
    }

    // 2a-VI. Write a loop that uses print to create the output shown below
    i = 100;
    while i >= 5 {
        println!("{}", i);
        i -= 5;
    }

    // 2b. For Lo
    // 2b. For Loops

    // 2b-I. Write some code that prompts the user for a number,
    // then shows a multiplication table up through 10 for that number.
    println!("Enter a number:");
    let mut num_mult = String::new();
    std::io::stdin().read_line(&mut num_mult).expect("Failed to read input.");
    let num_mult: i32 = num_mult.trim().parse().expect("Invalid number entered.");

    for i in 1..=10 {
        println!("{} x {} = {}", i, num_mult, i * num_mult);
    }

    // 2b-II. Create a for loop that uses print to create the output shown below.
    //
    // 1
    // 22
    // 333
    // 4444
    // 55555
    // 666666
    // 7777777
    // 88888888
    // 999999999
    for i in 1..=9 {
        for _ in 1..=i {
            print!("{}", i);
        }
        println!();
    }

    // 2c. break and continue

    // 2c-I. Write a program that prompts the user for a positive integer.
    // Next write a loop that prints out the numbers from the number the user
    // entered down to 1.
    println!("Enter a positive number:");
    let mut count_down = String::new();
    std::io::stdin().read_line(&mut count_down).expect("Failed to read input.");
    let count_down: i32 = count_down.trim().parse().expect("Invalid number entered.");

    while count_down >= 1 {
        println!("{}", count_down);
        count_down -= 1;
    }

    // 2c-II. Prompt the user to enter a positive number and write a
    // loop that counts from 0 to that number.
    println!("Enter a positive number:");
    let mut num_input = String::new();
    std::io::stdin().read_line(&mut num_input).expect("Failed to read input.");
    let num_input: i32 = num_input.trim().parse().expect("Invalid number entered.");

    if num_input > 0 {
        for i in 0..=num_input {
            println!("{}", i);
        }
    }

    // 2c-III. Prompt the user for an odd number between 1 and 50.
    // Use a loop and a break statement to continue prompting the user
    // if they enter invalid input.
    println!("Enter an odd number between 1 and 50:");
    let mut num_input = String::new();

    loop {
        std::io::stdin().read_line(&mut num_input).expect("Failed to read input.");
        let num_input: i32 = match num_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if num_input % 2 == 1 && num_input >= 1 && num_input <= 50 {
            break;
        } else {
            println!("Invalid input. Please enter an odd number between 1 and 50:");
            num_input.clear();
        }
    }

    println!("Number to skip is: {}", num_input);

    for i in 1..=50 {
        if i == num_input {
            println!("Yikes! Skipping number: {}", num_input);
            continue;
        }

        if i % 2 == 1 {
            println!("Here is an odd number: {}", i);
        }
    // 2b. For Loops

    // 2b-I. Write some code that prompts the user for a number,
    // then shows a multiplication table up through 10 for that number.
    println!("Enter a number:");
    let mut num_mult = String::new();
    std::io::stdin().read_line(&mut num_mult).expect("Failed to read input.");
    let num_mult: i32 = num_mult.trim().parse().expect("Invalid number entered.");

    for i in 1..=10 {
        println!("{} x {} = {}", i, num_mult, i * num_mult);
    }

    // 2b-II. Create a for loop that uses print to create the output shown below.
    //
    // 1
    // 22
    // 333
    // 4444
    // 55555
    // 666666
    // 7777777
    // 88888888
    // 999999999
    for i in 1..=9 {
        for _ in 1..=i {
            print!("{}", i);
        }
        println!();
    }

    // 2c. break and continue

    // 2c-I. Write a program that prompts the user for a positive integer.
    // Next write a loop that prints out the numbers from the number the user
    // entered down to 1.
    println!("Enter a positive number:");
    let mut count_down = String::new();
    std::io::stdin().read_line(&mut count_down).expect("Failed to read input.");
    let count_down: i32 = count_down.trim().parse().expect("Invalid number entered.");

    while count_down >= 1 {
        println!("{}", count_down);
        count_down -= 1;
    }

    // 2c-II. Prompt the user to enter a positive number and write a
    // loop that counts from 0 to that number.
    println!("Enter a positive number:");
    let mut num_input = String::new();
    std::io::stdin().read_line(&mut num_input).expect("Failed to read input.");
    let num_input: i32 = num_input.trim().parse().expect("Invalid number entered.");

    if num_input > 0 {
        for i in 0..=num_input {
            println!("{}", i);
        }
    }

    // 2c-III. Prompt the user for an odd number between 1 and 50.
    // Use a loop and a break statement to continue prompting the user
    // if they enter invalid input.
    println!("Enter an odd number between 1 and 50:");
    let mut num_input = String::new();

    loop {
        std::io::stdin().read_line(&mut num_input).expect("Failed to read input.");
        let num_input: i32 = match num_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if num_input % 2 == 1 && num_input >= 1 && num_input <= 50 {
            break;
        } else {
            println!("Invalid input. Please enter an odd number between 1 and 50:");
            num_input.clear();
        }
    }

    println!("Number to skip is: {}", num_input);

    for i in 1..=50 {
        if i == num_input {
            println!("Yikes! Skipping number: {}", num_input);
            continue;
        }

        if i % 2 == 1 {
            println!("Here is an odd number: {}", i);
        }
    }
        }
    // 3. FizzBuzz

    // One of the most common interview questions for entry-level programmers is the FizzBuzz test.
    // Developed by Imran Ghory, the test is designed to test basic looping and conditional logic skills.
    // Write a program that prints the numbers from 1 to 100.
    // For multiples of three print "Fizz" instead of the number.
    // For the multiples of five print "Buzz".
    // For numbers which are multiples of both three and five print "FizzBuzz".
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    // 4. Display a table of powers

    // Prompt the user to enter an integer.
    // Display a table of squares and cubes from 1 to the value entered.
    // Ask if the user wants to continue.
    // Assume that the user will enter valid data.
    // Only continue if the user agrees to.
    loop {
        println!("Enter an integer:");
        let mut num_in = String::new();
        std::io::stdin().read_line(&mut num_in).expect("Failed to read input.");
        let num_in: i32 = num_in.trim().parse().expect("Invalid number entered.");

        println!("number | squared | cubed");
        println!("------------------------");

        for i in 1..=num_in {
            let squared = i * i;
            let cubed = i * i * i;
            println!("{:<6} | {:<7} | {}", i, squared, cubed);
        }

        println!("Do you want to continue? (y/n)");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice = choice.trim().to_lowercase();

        if choice != "y" {
            break;
        }
    }

    // 5. Convert given number grades into letter grades.

    // Prompt the user for a numerical grade from 0 to 100.
    // Display the corresponding letter grade.
    // Prompt the user to continue.
    // Assume that the user will enter valid integers for the grades.
    // The application should only continue if the user agrees to.
    loop {
        println!("Enter a grade number (0-100):");
        let mut num_grd = String::new();
        std::io::stdin().read_line(&mut num_grd).expect("Failed to read input.");
        let num_grd: i32 = num_grd.trim().parse().expect("Invalid number entered.");

        let grade = match num_grd {
            88..=100 => "A",
            80..=87 => "B",
            67..=79 => "C",
            60..=66 => "D",
            0..=59 => "F",
            _ => "Invalid Grade",
        };

        println!("Letter Grade: {}", grade);

        println!("Do you want to continue? (y/n)");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice = choice.trim().to_lowercase();

        if choice != "y" {
            break;
        }
    }

    // 6. Create a list of dictionaries where each dictionary represents a book that you have read.
    // Each dictionary in the list should have the keys title, author, and genre.
    // Loop through the list and print out information about each book.

    struct Book {
        title: String,
        author: String,
        genre: String,
    }

    fn main() {
        let books: Vec<Book> = vec![
            Book {
                title: String::from("First 20 Minutes"),
                author: String::from("Gretchen Reynolds"),
                genre: String::from("Non-Fiction"),
            },
            Book {
                title: String::from("Harry Potter"),
                author: String::from("J.K. Rowling"),
                genre: String::from("Fiction"),
            },
            Book {
                title: String::from("Variable Star"),
                author: String::from("Robert A. Heinlein"),
                genre: String::from("Sci-Fi"),
            },
        ];

        for book in &books {
            println!("Title: {}", book.title);
            println!("Author: {}", book.author);
            println!("Genre: {}", book.genre);
            println!();
        }

        // a. Prompt the user to enter a genre,
        // then loop through your books list and print out the titles of all the books in that genre.
        println!("Enter a genre:");
        let mut genre_input = String::new();
        std::io::stdin()
            .read_line(&mut genre_input)
            .expect("Failed to read input.");
        let genre_input = genre_input.trim().to_lowercase();

        println!("Books in the {} genre:", genre_input);
        for book in &books {
            if book.genre.to_lowercase() == genre_input {
                println!("{}", book.title);
            }
        }
    }

