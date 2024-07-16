use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser as _;
use color_print::{cprint, cprintln};
use once_cell::sync::Lazy;
use regex::Regex;
use terso_file_checking::{Args, CheckError, Line, LineOrRange, RfidData};

fn main() -> Result<()> {
	let args = match Args::try_parse() {
		Ok(args) => args,
		Err(e) => {
			println!("{e}");
			return Ok(());
		}
	};

	let mut file_data = read_to_string(&args.input_file)?;

	file_data = file_data.lines().skip(16).collect::<Vec<_>>().join("\n");

	let parser = csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.flexible(false)
		.from_reader(file_data.as_bytes());

	let mut errors: Vec<CheckError> = Vec::new();

	let mut previous_line = None;

	let all_lines = parser
		.into_deserialize::<Line>()
		.collect::<Result<Vec<_>, _>>()?;

	for (line_no, parsed) in all_lines.iter().filter(|line| line.did_pass).enumerate() {
		errors.extend(check_epc(&parsed.epc_data, line_no));
		errors.extend(check_order(&parsed.epc_data, line_no, &mut previous_line));
	}

	errors.extend(check_range(&all_lines));

	for error in &errors {
		cprint!("Error - <r>{error}");
		if let Some(line_or_range) = error.line_or_range() {
			match line_or_range {
				LineOrRange::Line(line) => cprint!("<m> (line: </><y>{}</><m>)", line.get() + 17),
				LineOrRange::Range(range) => cprint!(
					"<m> (lines: </><y>{} - {}</><m>)",
					range.start + 17,
					range.end + 17
				),
			}
		}
		println!();
	}

	let error_count = errors.len();

	print!("Total errors - ");

	match error_count {
		0 => cprintln!("<g>0"),
		1..10 => cprintln!("<y>{}", error_count),
		rest => cprintln!("<r>{}", rest),
	};

	Ok(())
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^0{9}2\d{6}0{4}\d{4}$").unwrap());

fn check_epc(epc_data: &str, line: usize) -> Option<CheckError> {
	if RE.is_match(epc_data) {
		None
	} else {
		Some(CheckError::epc_did_not_match(line))
	}
}

fn check_order(
	rfid_data: &RfidData,
	line: usize,
	previous: &mut Option<(usize, RfidData)>,
) -> Option<CheckError> {
	if let Some((previous_line_no, previous_line)) = previous.replace((line, rfid_data.clone())) {
		let current_epc = rfid_data
			.serial_number()
			.map(|l| l as i16)
			.unwrap_or_default();
		let previous_epc = previous_line
			.serial_number()
			.map(|l| l as i16)
			.unwrap_or_default();

		if (current_epc - previous_epc).abs() != 1 {
			return Some(CheckError::epc_not_in_order(previous_line_no..line));
		}
	}

	None
}

fn check_range(lines: &[Line]) -> Option<CheckError> {
	let first = lines
		.iter()
		.find(|l| l.did_pass)
		.and_then(|l| l.epc_data.serial_number())
		.unwrap_or_default();
	let last = lines
		.iter()
		.filter(|l| l.did_pass)
		.last()
		.and_then(|l| l.epc_data.serial_number())
		.unwrap_or_default();

	((last..=first).count() != 2000).then(CheckError::tag_range_incomplete)
}
