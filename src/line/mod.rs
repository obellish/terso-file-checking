mod rfid_data;

use serde::{
	de::{Error as DeError, Visitor},
	Deserialize, Deserializer,
};

pub use self::rfid_data::RfidData;

#[derive(Debug, Clone)]
pub struct Line {
	pub timestamp: String,
	pub did_pass: bool,
	pub epc_data: RfidData,
	pub tid_data: RfidData,
}

impl<'de> Deserialize<'de> for Line {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct(
			"Line",
			&["timestamp", "did_pass", "epc_data", "tid_data"],
			LineVisitor,
		)
	}
}

struct LineVisitor;

const INVALID_LENGTH_MESSAGE: &str = "struct Line with 4 elements";

impl<'de> Visitor<'de> for LineVisitor {
	type Value = Line;

	fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("struct Line")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::SeqAccess<'de>,
	{
		let Some(timestamp) = seq.next_element()? else {
			return Err(DeError::invalid_length(0, &INVALID_LENGTH_MESSAGE));
		};

		let Some(Passed(did_pass)) = seq.next_element()? else {
			return Err(DeError::invalid_length(1, &INVALID_LENGTH_MESSAGE));
		};

		let Some(epc_data) = seq.next_element()? else {
			return Err(DeError::invalid_length(2, &INVALID_LENGTH_MESSAGE));
		};

		let Some(tid_data) = seq.next_element()? else {
			return Err(DeError::invalid_length(3, &INVALID_LENGTH_MESSAGE));
		};

		Ok(Line {
			timestamp,
			did_pass,
			epc_data,
			tid_data,
		})
	}
}

struct Passed(bool);

impl<'de> Deserialize<'de> for Passed {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(PassedVisitor)
	}
}

struct PassedVisitor;

impl<'de> Visitor<'de> for PassedVisitor {
	type Value = Passed;

	fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("PASS or FAIL")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		match v {
			"PASS" => Ok(Passed(true)),
			"FAIL" => Ok(Passed(false)),
			_ => Err(DeError::custom("invalid passed record")),
		}
	}
}
