mod check_error;
mod line;

use std::path::PathBuf;

use clap::Parser;

pub use self::{
	check_error::{CheckError, ErrorType, LineOrRange},
	line::{Line, RfidData},
};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
#[repr(transparent)]
pub struct Args {
	/// The input file to run through
	#[arg(short, long, value_name = "FILE")]
	pub input_file: PathBuf,
}
