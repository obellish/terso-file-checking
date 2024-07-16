mod rfid_data;

use serde::{de::Visitor, Deserialize, Deserializer};

pub use self::rfid_data::RfidData;

#[derive(Debug, Clone, Deserialize)]
pub struct Line {
	pub timestamp: String,
	#[serde(deserialize_with = "deserialize_bool")]
	pub did_pass: bool,
	pub epc_data: RfidData,
	pub tid_data: RfidData,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	deserializer.deserialize_str(BoolVisitor)
}

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
	type Value = bool;

	fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("PASS or FAIL")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		match v {
			"PASS" => Ok(true),
			"FAIL" => Ok(false),
			_ => Err(serde::de::Error::custom("Invalid pass record")),
		}
	}
}
