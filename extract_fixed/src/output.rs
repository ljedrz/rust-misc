use std::io;
use std::io::{BufRead, BufReader, Write, Seek};
use std::io::SeekFrom::{Start, Current};
use std::fs::File;
use std::path::Path;
use self::PadSide::*;
use self::Content::*;
use definitions::Definition;
use interpreter::extract_line;

#[derive(PartialEq)]
pub enum PadSide {
    Left,
    Right
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Content {
    Bytes,
    Chars
}

pub trait Padded {
	fn pad(&self, pad_char: char, length: usize, side: PadSide, content: Content) -> String;
}

impl Padded for str {
	fn pad(&self, pad_char: char, length: usize, side: PadSide, content: Content) -> String {
		let mut padded_str = String::new();
		let pad_len = match content {
			Bytes => length - self.len(),
			Chars => length - self.chars().map(|c| c.len_utf8()).fold(0, |sum, len| sum + len)
		};

		if side == Right { padded_str.push_str(self) }
		for _ in 0..pad_len { padded_str.push(pad_char) }
		if side == Left { padded_str.push_str(self) }

		padded_str
	}
}

pub fn display_extract(input_file_name: &str, definition: &Definition) -> io::Result<()>  {
	let input_file = try!(File::open(Path::new(input_file_name)));
	let max_name_len = definition.fields.iter().map(|field| field.len()).max().unwrap();
	let max_value_len = definition.lengths.iter().max().unwrap();
	let mut index;
	let mut line_extract;

	for (line_index, line) in BufReader::new(input_file).lines().enumerate() {
		index = format!("{}", line_index + 1);
		println!("line {} {}\n", index, "".pad('-', max_name_len + max_value_len - 2 - index.len(), Right, Chars));
		line_extract = extract_line(&line.unwrap(), &definition);
		for (field_index, ref field_name) in definition.fields.iter().enumerate() {
			println!("{}: \"{}\"", field_name.pad(' ', max_name_len, Right, Chars), &line_extract[field_index]);
		}
		println!("");
	}

	Ok(())
}

pub fn export_as_rows(input_file_name: &str, definition: &Definition) -> io::Result<()> {
	let input_file = try!(File::open(Path::new(input_file_name)));
	let mut target_file = try!(File::create(Path::new(&format!("{}{}", "extract_", input_file_name))));

	try!(writeln!(&mut target_file, "{}", &definition.fields.join("\t")));
	for line in BufReader::new(input_file).lines() {
		try!(writeln!(&mut target_file, "{}", &extract_line(&line.unwrap(), &definition).join("\t")));
	}

	Ok(())
}
// TODO: proper file reload
pub fn export_as_cols(input_file_name: &str, definition: &Definition) -> io::Result<()> {
	let mut input_file = try!(File::open(Path::new(input_file_name)));
	let mut target_file = try!(File::create(Path::new(&format!("{}{}", "extract_", input_file_name))));
	let mut remaining_line_space;
	let mut line_extract: Vec<String>;
	let mut curr_offset: i64;

	let input_file_line_count = BufReader::new(input_file).lines().count();

	for (field_ind, field) in definition.fields.iter().enumerate() {
		remaining_line_space = (definition.lengths[field_ind] + 1) * input_file_line_count as usize;
		try!(write!(&mut target_file, "{}\t{}", field, "".pad(' ', remaining_line_space, Right, Chars)));
	}

	input_file = try!(File::open(Path::new(input_file_name)));
	try!(target_file.seek(Start(0)));
	for (line_ind, line) in BufReader::new(input_file).lines().enumerate() {
		line_extract = extract_line(&line.unwrap(), &definition);
		try!(target_file.seek(Start(0)));
		curr_offset = 0;
		for (field_ind, field_len) in definition.lengths.iter().enumerate() {
			curr_offset += definition.fields[field_ind].len() as i64 + 1 + (*field_len as i64 + 1) * line_ind as i64;
			remaining_line_space = (input_file_line_count as usize - line_ind) * (*field_len + 1);
			try!(target_file.seek(Start(curr_offset as u64)));
			try!(write!(&mut target_file, "{}\t", line_extract[field_ind]));
			curr_offset += remaining_line_space as i64;
			if line_ind == input_file_line_count as usize - 1 {
				try!(target_file.seek(Current(-1)));
				try!(write!(&mut target_file, "\n"));
			};
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::PadSide::*;
	use super::Content::*;
	use super::Padded;

	#[test]
	fn byte_padding_ansi_r() {
		assert_eq!("derp   ", "derp".pad(' ', 7, Right, Bytes))
	}

	#[test]
	fn byte_padding_ansi_l() {
		assert_eq!("   derp", "derp".pad(' ', 7, Left, Bytes))
	}

	#[test]
	fn char_padding_ansi_r() {
		assert_eq!("derp ", "derp".pad(' ', 5, Right, Chars))
	}

	#[test]
	fn char_padding_ansi_l() {
		assert_eq!(" derp", "derp".pad(' ', 5, Left, Chars))
	}

	#[test]
	fn byte_padding_wide_r() {
		assert_eq!("pâté  ", "pâté".pad(' ', 8, Right, Bytes))
	}

	#[test]
	fn byte_padding_wide_l() {
		assert_eq!("  pâté", "pâté".pad(' ', 8, Left, Bytes))
	}

	#[test]
	fn char_padding_wide_r() {
		assert_eq!("pâté  ", "pâté".pad(' ', 8, Right, Chars))
	}

	#[test]
	fn char_padding_wide_l() {
		assert_eq!("  pâté", "pâté".pad(' ', 8, Left, Chars))
	}
}