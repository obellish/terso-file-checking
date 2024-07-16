use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
	num::NonZeroUsize,
	ops::Range,
};

#[derive(Debug)]
pub struct CheckError {
	kind: ErrorType,
	line: Option<LineOrRange>,
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
	pub fn with_line(kind: ErrorType, line: usize) -> Self {
		Self {
			kind,
			line: NonZeroUsize::new(line).map(LineOrRange::Line),
			source: None,
		}
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn with_range(kind: ErrorType, range: Range<usize>) -> Self {
		Self {
			kind,
			line: Some(LineOrRange::Range(range)),
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
	pub fn with_line_and_source(
		kind: ErrorType,
		line: usize,
		source: Box<dyn StdError + Send + Sync>,
	) -> Self {
		Self {
			kind,
			line: NonZeroUsize::new(line).map(LineOrRange::Line),
			source: Some(source),
		}
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub fn epc_did_not_match(line: usize) -> Self {
		Self::with_line(ErrorType::EpcDidNotMatch, line)
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn epc_not_in_order(range: Range<usize>) -> Self {
		Self::with_range(ErrorType::EpcNotInOrder, range)
	}

	#[must_use = "constructing an error does nothing if unused"]
	pub const fn tag_range_incomplete() -> Self {
		Self::new(ErrorType::TagRangeIncomplete)
	}

	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> ErrorType {
		self.kind
	}

	#[must_use]
	pub fn line_or_range(&self) -> Option<LineOrRange> {
		self.line.clone()
	}

	#[must_use = "retrieving the line has no effect if left unused"]
	pub const fn line(&self) -> Option<usize> {
		if let Some(LineOrRange::Line(line)) = self.line {
			Some(line.get())
		} else {
			None
		}
	}

	#[must_use = "retrieving the range has no effect if left unused"]
	pub fn range(&self) -> Option<Range<usize>> {
		if let Some(LineOrRange::Range(range)) = &self.line {
			Some(range.clone())
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
			ErrorType::EpcDidNotMatch => f.write_str("epc did not match expected format"),
			ErrorType::EpcNotInOrder => f.write_str("epc was not in order"),
			ErrorType::TagRangeIncomplete => f.write_str("tag range is incomplete"),
		}
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

#[derive(Debug, Clone)]
pub enum LineOrRange {
	Line(NonZeroUsize),
	Range(Range<usize>),
}
