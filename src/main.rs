use std::io; // input/output library (from std : standard library) 
use rand::Rng;
use std::cmp::Ordering;

// prelude ^^^ (the set of items defined in the stand library that it brings into the scope of every program

fn main() { // entry point of the program
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	// println!("The secret number is: {secret_number}");

	loop {
	println!("Please input your guess!");

	let mut guess = String::new(); // "let" is used to create variable, "mut" makes the variable mutable, "String::new" returns string
	// thus, this line creates a mutable variable that is bound to a new, empty instance of type String

	io::stdin() // recieves user input
		.read_line(&mut guess) // store user input (line) into guess (passed by reference &) 
		.expect("Failed to read line"); // catch error 

	let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	};
	// guess.trim().parse(). eliminate whitespace 
	// "u32" = 32-bit unsigned integer 
	
	println!("You guessed: {}", guess); // "{}" is the placeholder for the value of a variable
	
	match guess.cmp(&secret_number) { // cmp guess to secret_number
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => { 
			println!("You win!");
			break;
		}
	}
	}
}
