use std::io;
use rand::Rng;

fn main() {
	println!("guess the number");
	println!("Please input your guess:");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	let mut guess = String::new();

	println!("The secret number is: {secret_number}");

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read");

	println!("you guessed: {guess}");
}