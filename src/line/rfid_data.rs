use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	ops::Deref,
};

use serde::{
	de::{Error as DeError, Unexpected, Visitor},
	Deserialize,
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct RfidData {
	pub error_code: Option<ErrorCode>,
	pub data: String,
}

impl Debug for RfidData {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut debug = f.debug_struct("RfidData");

		if let Some(error_code) = self.error_code {
			debug.field("error_code", &error_code)
		} else {
			debug.field("data", &self.data)
		}
		.finish()
	}
}

impl Deref for RfidData {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<'de> Deserialize<'de> for RfidData {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(RfidDataVisitor)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
	DidNotRespondToQuery,
	DidNotRespondToCommand,
	InventoryFailed,
	TagError(TagError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagError {
	Other,
	MemoryOverrun,
	MemoryLocked,
	InsufficientPower,
	NonSpecific,
}

struct RfidDataVisitor;

impl<'de> Visitor<'de> for RfidDataVisitor {
	type Value = RfidData;

	fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("struct RfidData")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let mut parts = v.split('/');

		let error_code = parts
			.next()
			.ok_or_else(|| DeError::missing_field("error_code"))?;
		let rfid_data = parts.next().ok_or_else(|| DeError::missing_field("data"))?;

		let error_code = match error_code {
			"0" => None,
			"1" => Some(ErrorCode::DidNotRespondToQuery),
			"2" => Some(ErrorCode::DidNotRespondToCommand),
			"3" => Some(ErrorCode::InventoryFailed),
			"4" => Some(ErrorCode::TagError(TagError::Other)),
			"52" => Some(ErrorCode::TagError(TagError::MemoryOverrun)),
			"68" => Some(ErrorCode::TagError(TagError::MemoryLocked)),
			"180" => Some(ErrorCode::TagError(TagError::InsufficientPower)),
			"244" => Some(ErrorCode::TagError(TagError::NonSpecific)),
			_ => {
				return Err(DeError::invalid_value(
					Unexpected::Str(error_code),
					&"A valid error code",
				))
			}
		};

		Ok(RfidData {
			error_code,
			data: rfid_data.to_owned(),
		})
	}
}
