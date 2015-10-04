fn csv_to_xml(input: &str, delim: char, root_name: &str, row_name: &str, col_name: &str) -> String {
	let mut output = String::new();

	output.open_elem(root_name);

	for line in input.lines_any() {
		output.open_elem(row_name);
		for value in line.split(delim) { output.full_elem(col_name, value);	}
		output.close_elem(row_name);
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
    use super::csv_to_xml;

    #[test]
    fn csv_to_xml_test() {
		let test_str = "herp;derp\nhurr;durr";
		let target = "<root><row><col>herp</col><col>derp</col></row>\
			<row><col>hurr</col><col>durr</col></row></root>";

		assert_eq!(csv_to_xml(test_str, ';', "root", "row", "col"), target);
	}
}