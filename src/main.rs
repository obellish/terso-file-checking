use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser as _;
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

	for (line_no, parsed) in parser.into_deserialize::<Line>().enumerate() {
		let parsed = parsed?;

		if parsed.did_pass {
			if let Some(e) = check_epc_prefix(&parsed.epc_data, line_no) {
				errors.push(e);
			}
		}
	}

	for error in errors {
		println!("Error - {error}");
	}

	Ok(())
}

fn check_epc_prefix(epc_data: &str, line: usize) -> Option<CheckError> {
	if &(epc_data[..10]) != "0000000002" {
		return Some(CheckError::epc_prefix_did_not_match(line + 17));
	}

	None
}
