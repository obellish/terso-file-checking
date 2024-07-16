use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult, Write as _},
	num::NonZeroUsize,
};

#[derive(Debug)]
pub struct CheckError {
	kind: ErrorType,
	line: Option<NonZeroUsize>,
	source: Option<Box<dyn StdError + Send + Sync>>,
}

impl CheckError {
	#[must_use = "constructing an error does nothing if unused"]
	pub const fn new(kind: ErrorType) -> Self {
		Self {
			kind,
			line: None,
			source: None,
		}
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn with_line(kind: ErrorType, line: usize) -> Self {
		Self {
			kind,
			line: NonZeroUsize::new(line),
			source: None,
		}
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn with_source(kind: ErrorType, source: Box<dyn StdError + Send + Sync>) -> Self {
		Self {
			kind,
			line: None,
			source: Some(source),
		}
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn with_line_and_source(
		kind: ErrorType,
		line: usize,
		source: Box<dyn StdError + Send + Sync>,
	) -> Self {
		Self {
			kind,
			line: NonZeroUsize::new(line),
			source: Some(source),
		}
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn epc_did_not_match(line: usize) -> Self {
		Self::with_line(ErrorType::EpcDidNotMatch, line)
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn epc_not_in_order(line: usize) -> Self {
		Self::with_line(ErrorType::EpcNotInOrder, line)
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn tag_range_incomplete() -> Self {
		Self::new(ErrorType::TagRangeIncomplete)
	}

	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> ErrorType {
		self.kind
	}

	#[must_use = "retrieving the line has no effect if left unused"]
	pub const fn line(&self) -> Option<usize> {
		if let Some(line) = self.line {
			Some(line.get())
		} else {
			None
		}
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
			ErrorType::EpcDidNotMatch => {
				f.write_str("epc did not match expected format")?;
			}
			ErrorType::EpcNotInOrder => {
				f.write_str("epc was not in order")?;
			}
			ErrorType::TagRangeIncomplete => {
				f.write_str("tag range is incomplete")?;
			}
		}

		if f.alternate() {
			if let Some(line) = self.line {
				f.write_str(" (line: ")?;
				Display::fmt(&line, f)?;
				f.write_char(')')?;
			}
		}

		Ok(())
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
	EpcDidNotMatch,
	EpcNotInOrder,
	TagRangeIncomplete,
}
