// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::io;
use rand::Rng;
use colored::*;

fn int_to_norsk(number: i32) -> String {
	let units = ["", "én", "to", "tre", "fire", "fem", "seks", "sju", "åtte", "ni"];
	let teens = ["ti", "elleve", "tolv", "tretten", "fjorten", "femten", "seksten", "sytten", "atten", "nitten"];
	let tens  = ["tjue", "tretti", "førti", "femti", "seksti", "sytti", "åtti", "nitti"];

	let norsk = match number {
		0..=9   => units[number as usize].to_string(),
		10..=19 => teens[(number-10) as usize].to_string(),
		20..=99 => {
			let tens_string: &str = tens[(number/10-2) as usize];
			let units_string: &str = units[(number % 10) as usize];
			let together = format!("{}{}", tens_string, units_string);
			together
		},
		_ => "hundre".to_string()
	};
	return norsk
}

fn int_to_norsk_nodanglybits(number: i32) -> String {
	let units = ["", "en", "to", "tre", "fire", "fem", "seks", "sju", "atte", "ni"];
	let teens = ["ti", "elleve", "tolv", "tretten", "fjorten", "femten", "seksten", "sytten", "atten", "nitten"];
	let tens  = ["tjue", "tretti", "forti", "femti", "seksti", "sytti", "atti", "nitti"];

	let norsk = match number {
		0..=9   => units[number as usize].to_string(),
		10..=19 => teens[(number-10) as usize].to_string(),
		20..=99 => {
			let tens_string: &str = tens[(number/10-2) as usize];
			let units_string: &str = units[(number % 10) as usize];
			let together = format!("{}{}", tens_string, units_string);
			together
		},
		_ => "hundre".to_string()
	};
	return norsk
}

fn main() {
	println!("\n\n\n{}", "Practice Norwegian numbers!".bright_yellow().bold());
	println!("{}", "(To quit, press ctrl+c)".bright_red());
	loop {
		let number = rand::thread_rng().gen_range(1, 101);
		println!("{} {} {}", "Write".bright_cyan(), number, "in Norwegian:".bright_cyan());
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess = guess.trim();

		if guess == int_to_norsk(number) {
			println!("{}", "Correct!".bright_green());
		} else if guess == int_to_norsk_nodanglybits(number) {
			println!("{}, but dont forget the accents! ({})", "Correct".bright_yellow(), int_to_norsk(number).bright_green());
		} else {
			println!("{}, it was {}", "Not correct".bright_red(), int_to_norsk(number).bright_green());
			break;
		}
	}
}
