use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult, Write as _},
};

#[derive(Debug)]
pub struct CheckError {
	kind: ErrorType,
	line: usize,
	source: Option<Box<dyn StdError + Send + Sync>>,
}

impl CheckError {
	#[must_use]
	pub fn new(kind: ErrorType, line: usize) -> Self {
		Self {
			kind,
			line,
			source: None,
		}
	}

	#[must_use]
	pub fn with_source(
		kind: ErrorType,
		line: usize,
		source: Box<dyn StdError + Send + Sync>,
	) -> Self {
		Self {
			kind,
			line,
			source: Some(source),
		}
	}

	#[must_use]
	pub fn epc_prefix_did_not_match(line: usize) -> Self {
		Self::new(ErrorType::EpcPrefixDidNotMatch, line)
	}

	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> ErrorType {
		self.kind
	}

	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn StdError + Send + Sync>> {
		self.source
	}

	#[must_use = "consuming the error into its parts has no effect if left unused"]
	pub fn into_parts(self) -> (ErrorType, Option<Box<dyn StdError + Send + Sync>>) {
		(self.kind, self.source)
	}
}

impl Display for CheckError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self.kind {
			ErrorType::EpcPrefixDidNotMatch => {
				f.write_str("epc prefix did not match expected format")?;
			}
		}

		f.write_str(" (line: ")?;
		Display::fmt(&self.line, f)?;
		f.write_char(')')
	}
}

impl StdError for CheckError {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn StdError + 'static))
	}
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
	EpcPrefixDidNotMatch,
}
