use rand::Rng; // use the rand package for random number generator
use std::io; //  bring the io input/output library into scope

fn main(){ // entry point of every rust code
    loop{ // this is a loop statement
        println!("-----GUESSING GAME-----"); // println! is a macro that prints a string to the screen
        println!("Enter the number: ");
        // let mut guess = String::new(); // We use the let statement to create the variable, and 
        // String::new, a function that returns a new instance of a String. line has created a mutable variable that is currently bound to a new, empty instance of a String
        let secret = rand::thread_rng().gen_range(1..=10); // use the random package to generate random number
        // io::stdin() //The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
        //     .read_line(&mut guess)  //  calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in.
        //     .expect("Failed to read the line"); // If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if its ok, then its a num
            Err(_) => continue, // if not, dont panick, just continue
        };
        //let guess: u32 = guess.trim().parse().expect("Please type a number!"); // CHECK NOTE
        println!("You guess the number: {guess}"); //This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place
        println!("The secret number was: {secret}");
        if guess == secret{
            println!("You guessed correctly!");
            break;
        } else if guess < secret{
            println!("Your guess is lower");
        } else{
            println!("Your guess is higher");
        }
    }
}

// NOTE:
//We bind this new variable to the expression guess.trim().parse(). The guess in the expression refers to the original guess variable that contained the input as a string. The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data. The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer

// intead of using if else we can use 
//  use std::cmp::Ordering;
//  match guess.cmp(&secret_number) {
//    Ordering::Less => println!("Too small!"),
//    Ordering::Greater => println!("Too big!"),
//    Ordering::Equal => {
//        println!("You win!");
//       break;
//    }
//}