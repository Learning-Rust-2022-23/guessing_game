use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("guess the number");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Please input your guess:");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read");
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("you guessed: {guess}");

		match guess.cmp(&secret_number){
			Ordering::Less => println!("hint, the number is too small"),
			Ordering::Greater => println!("hint, the number is too high"),
			Ordering::Equal => {
				println!("You win");
				break;
			},
		}
	}

}