use core::num;
use std::io;

use rand::RngExt;

use std::cmp::Ordering;





fn main() {
    // let mut name = "Seth"; // variables are immutables by default 
    // let lastName = "Tang";
    // // name = "New";
    // println!("Hello, {name} {}!", lastName.to_lowercase());
    

    // let newName = name;
    // print!("New name : {}", newName);

    // let data = [1,2,3,4,5];
    // println!("{data:?}"); // debug format (to see data)


    // // working with string 
    // let firstName = String::from("Seth"); // more dynamicness when we don't want to know the input of the user ahead of time 

    // let empty = String::new();
    // println!("{empty}");

    // let mut name = String::new();

    // io::stdin().
    //     read_line(&mut name).
    //     expect("Error reading input"); // a simple/naïve way to handle the error 
    

    // println!("Hello {}. Welcome!", name.trim_end());

    // println!("{firstName}");
    let mut how_many = String::new();
    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");
    // 2 ^8 = 255 guesses 

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::rng().random_range(1..=10));
    }
    println!("{correct:?}");
    // let correct = rand::rng().random_range(1..=10);
    // println!("correct  = {correct}");
    // println!("Hey, guess a number 1-10: ");

    let mut guesses_made = 0;

    while guesses_made < num_guesses {
        println!("Hey, guess a number 1-10:");
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Error reading input number");

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again! {e}");
                continue;
            }
        };

        // let mut message = if correct < guess {
            //     String::from("You guessed too high!")
            // } else if correct > guess {
        //     String::from("You guessed too low!")
        // } else {
            //     String::from("You guessed correct!")
            // };
            
            // we can simplify our code below:
            // 
            // 
            match guess.cmp(&correct[guesses_made as usize]) {
                Ordering::Greater => println!("You guessed too high!"),
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Equal => {
                println!("You guessed correct!");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Let's now try the next number.");
                }
                // break;
            }
        };
        // println!("{message}");
        // message += message;
        // println!("You guessed {}", guess.trim());
    }
    println!("Thanks for playing! The correct answers were:");
    for item in correct {
        println!("{item}");
    }
    
    
    

}

// a package can contain multiple crates 