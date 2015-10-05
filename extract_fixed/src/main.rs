use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
mod definitions;
use definitions::load_definitions;
mod interpreter;
use interpreter::identify_file;
mod output;
use output::{display_extract, export_as_rows, export_as_cols};

#[cfg(not(test))]
fn main() {
	let (file_name, extract_type) = get_user_input().ok().expect("error: invalid input");
	let definitions = load_definitions(Path::new("definitions")).ok().expect("error: definition folder not found");
	let definition = identify_file(&file_name, &definitions).unwrap();

	println!("the file contains \"{}\" records (line size: {}B)\n", definition.name, definition.lengths.iter().fold(0, |sum, len| sum + len));

	match extract_type {
		1 => display_extract(&file_name, &definition).ok().expect("error: unable to display the extract"),
		2 => export_as_cols(&file_name, &definition).ok().expect("error: unable to save the extract"),
		3 => export_as_rows(&file_name, &definition).ok().expect("error: unable to save the extract"),
		_ => println!("error: invalid choice")
	}

	exit().unwrap();
}

#[cfg(not(test))]
fn get_user_input() -> io::Result<(String, u8)> {
	let mut file_name = String::new();
	let mut extract_type = String::new();

	print!("enter the name of the file to interpret: ");
	try!(io::stdout().flush());
	try!(io::stdin().read_line(&mut file_name));
	println!("\nwould you like to:");
	println!("(1) display records in a list");
	println!("(2) export records as columns");
	println!("(3) export records as rows");
	print!(">");
	try!(io::stdout().flush());
	try!(io::stdin().read_line(&mut extract_type));
	println!("");

	Ok((file_name.trim_right().to_owned(), extract_type.trim_right().parse::<u8>().ok().expect("error: invalid number supplied")))
}

#[cfg(not(test))]
fn exit() -> io::Result<usize> {
	print!("done; press any key to exit");
	try!(io::stdout().flush());
	let mut buffer = String::new();
	let sink = io::stdin().read_line(&mut buffer);

	sink
}