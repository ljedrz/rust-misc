use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::fs::File;
use definitions::{Definition, pick_definition};

pub fn identify_file(input_file_name: &str, definitions: &Vec<Definition>) -> io::Result<Definition> {
	let input_file = try!(File::open(Path::new(input_file_name)));
	let mut first_line = String::new();

	try!(BufReader::new(input_file).read_line(&mut first_line));

	pick_definition(&first_line, &definitions).ok_or(Error::new(ErrorKind::Other, "applicable definition not found"))
}

pub fn extract_line(fixed_line: &str, definition: &Definition) -> Vec<String> {
	let mut line_extract: Vec<String> = Vec::with_capacity(definition.fields.len());

	let mut position = 0;
	for &field_size in definition.lengths.iter() {
		line_extract.push(fixed_line[position..position + field_size].to_owned());
		position += field_size;
	}

	line_extract
}

#[cfg(test)]
mod test {

}