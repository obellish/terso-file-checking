use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser as _;
use color_print::cprint;
use once_cell::sync::Lazy;
use regex::Regex;
use terso_file_checking::{Args, CheckError, Line};

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

	for (line_no, parsed) in all_lines.iter().enumerate() {
		if parsed.did_pass {
			if let Some(e) = check_epc(&parsed.epc_data, line_no) {
				errors.push(e);
				continue;
			}
		}
	}

	for (line_no, parsed) in all_lines.iter().filter(|line| line.did_pass).enumerate() {
		if let Some(previous_line) = previous_line.replace(parsed.clone()) {
			let current_epc = parsed
				.epc_data
				.serial_number()
				.map(|l| l as i16)
				.unwrap_or_default();
			let previous_epc = previous_line
				.epc_data
				.serial_number()
				.map(|l| l as i16)
				.unwrap_or_default();

			if (current_epc - previous_epc).abs() != 1 {
				errors.push(CheckError::epc_not_in_order(line_no));
			}
		}
	}

	let first = all_lines
		.iter()
		.find(|l| l.did_pass)
		.and_then(|l| l.epc_data.serial_number())
		.unwrap_or_default();

	let last = all_lines
		.iter()
		.filter(|l| l.did_pass)
		.last()
		.and_then(|l| l.epc_data.serial_number())
		.unwrap_or_default();

	if (first..=last).count() != 2000 {
		errors.push(CheckError::tag_range_incomplete());
	}

	for error in errors {
		cprint!("Error - <r>{error}");
		if let Some(line) = error.line() {
			cprint!("<m> (line: </><y>{}</><m>)", line + 17);
		}
		println!();
	}

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
