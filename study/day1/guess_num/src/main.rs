use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess num game!");

	let rand_num = rand::thread_rng().gen_range(1, 101);

	loop {
	    println!("please input your guess:");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("fail to read line");
	
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("your guess {}", guess);

		match guess.cmp(&rand_num) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("congratulation, You win!");
				break;
			}
		}
	}
}
