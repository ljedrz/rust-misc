use std::io;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::fs::File;

#[derive(Clone)]
pub struct Definition {
	pub name: String,
	pub fields: Vec<String>,
	pub lengths: Vec<usize>
}

fn load_definition(definition_path: &Path) -> io::Result<Definition> {
	let mut fields_file = String::new();
	let fields_pairs: Vec<(&str, &str)>;

	try!(File::open(Path::new(&definition_path)).unwrap().read_to_string(&mut fields_file));

	fields_pairs = fields_file.lines_any().map(|line| {let mut pair = line.split("\t"); (pair.next().unwrap(), pair.next().unwrap())} ).collect();

	Ok(Definition {
		name: definition_path.file_stem().unwrap().to_str().unwrap().to_owned(),
		fields: fields_pairs.iter().map(|&(name, _)| name.to_owned()).collect(),
		lengths: fields_pairs.iter().map(|&(_, length)| length.parse::<usize>().unwrap()).collect()
	})
}

pub fn load_definitions(definitions_path: &Path) -> io::Result<Vec<Definition>> {
	let definition_files = fs::read_dir(definitions_path).unwrap();
	let mut definition_path;
	let mut definitions = Vec::with_capacity(fs::read_dir(definitions_path).unwrap().count()); // TODO: anything more elegant?

	for file in definition_files {
		definition_path = file.unwrap().path();
		definitions.push(load_definition(&definition_path).unwrap());
	}

	Ok(definitions)
}

pub fn pick_definition(line: &str, definitions: &Vec<Definition>) -> Option<Definition> {
	definitions.iter().find(|def| def.lengths.iter().fold(0, |sum, len| sum + len) == line.trim_right_matches(|c| c == '\r' || c == '\n').len()).map(|def| def.to_owned())
}

#[cfg(test)]
mod test {

}