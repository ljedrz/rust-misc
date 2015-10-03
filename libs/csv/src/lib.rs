fn csv_to_xml(input: &str, delim: char, root_name: &str, row_name: &str, col_name: &str) -> String {
	let mut output = String::new();

	output.push('<');
	output.push_str(root_name);
	output.push('>');

	for line in input.lines_any() {
		output.push('<');
		output.push_str(row_name);
		output.push('>');
		for value in line.split(delim) {
			output.push('<');
			output.push_str(col_name);
			output.push('>');
			output.push_str(value);
			output.push_str("</");
			output.push_str(col_name);
			output.push('>');
		}
		output.push_str("</");
		output.push_str(row_name);
		output.push('>');
	}

	output.push_str("</");
	output.push_str(root_name);
	output.push('>');

	output
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