#![allow(dead_code)]

fn vectorize_csv(csv: &str, delim: char) -> Vec<Vec<String>> {
	let csv_lines: Vec<&str> = csv.lines_any().collect();
	let mut csv_vec = Vec::with_capacity(csv_lines.len());

	for line in csv_lines {
		csv_vec.push(line.split(|c| c == delim).map(|s| s.to_owned()).collect());
	}

	csv_vec
}

fn csv_to_xml(input: &str, delim: char, root_name: &str, header: Option<&[&str]>) -> String {
	let mut output = String::new();

	output.open_elem(root_name);

	for line in input.lines_any() {
		output.open_elem("line");
		for (index, value) in line.split(delim).enumerate() {
			match header {
				Some(fields) => output.full_elem(fields[index], value),
				None => output.full_elem("field", value)
			};
		}
		output.close_elem("line");
	}

	output.close_elem(root_name);

	output
}

trait XmlStr {
	fn open_elem(&mut self, elem_name: &str);
	fn close_elem(&mut self, elem_name: &str);
	fn full_elem(&mut self, elem_name: &str, elem_content: &str);
}

impl XmlStr for String {
	fn open_elem(&mut self, elem_name: &str) {
		self.push('<');
		self.push_str(elem_name);
		self.push('>');
	}

	fn close_elem(&mut self, elem_name: &str) {
		self.push_str("</");
		self.push_str(elem_name);
		self.push('>');
	}

	fn full_elem(&mut self, elem_name: &str, elem_content: &str) {
		self.open_elem(elem_name);
		self.push_str(elem_content);
		self.close_elem(elem_name);
	}
}

#[cfg(test)]
mod tests {
    use super::{vectorize_csv, csv_to_xml};

	#[test]
	fn csv_vectorization_test() {
		let vec1 = vec![vec!["herp", "derp", "durr"], vec!["1", "2", ""], vec!["", "b", "c"]];
		let vec2 = vectorize_csv("herp;derp;durr\n1;2;\r\n;b;c", ';');

		assert_eq!(vec1, vec2);
	}

    #[test]
    fn csv_to_xml_header() {
		let test_str = "herp;derp\nhurr;durr";
		let target = "<root><line><field1>herp</field1><field2>derp</field2></line>\
			<line><field1>hurr</field1><field2>durr</field2></line></root>";

		assert_eq!(csv_to_xml(test_str, ';', "root", Some(&["field1", "field2"])), target);
	}

	#[test]
	fn csv_to_xml_noheader() {
		let test_str = "herp;derp\nhurr;durr";
		let target = "<root><line><field>herp</field><field>derp</field></line>\
			<line><field>hurr</field><field>durr</field></line></root>";

		assert_eq!(csv_to_xml(test_str, ';', "root", None), target);
	}
}