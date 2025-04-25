use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub fn date(value: &DateTime<Utc>, _: &dyn askama::Values) -> askama::Result<String> {
	Ok(value.format("%Y-%m-%d %H:%M").to_string())
}

pub fn default_string(value: &str, _: &dyn askama::Values, default_value: &str) -> askama::Result<String> {
	if value.is_empty() {
		Ok(default_value.to_string())
	} else {
		Ok(value.to_string())
	}
}

pub fn default_option(value: &Option<String>, _: &dyn askama::Values, default_value: &str) -> askama::Result<String> {
	match value {
		Some(s) if !s.is_empty() => Ok(s.clone()),
		_ => Ok(default_value.to_string()),
	}
}

pub fn default_int<'a>(value: &i32, _: &dyn askama::Values, default_value: &'a i32) -> askama::Result<String> {
	if *value == 0 {
		Ok(default_value.to_string())
	} else {
		Ok(value.to_string())
	}
}

pub fn default_decimal<'a>(value: &Decimal, _: &dyn askama::Values, default_value: &'a f64) -> askama::Result<String> {
	if value.is_zero() {
		Ok(default_value.to_string())
	} else {
		Ok(value.to_string())
	}
}

pub fn option_uuid_eq(option_id: &Option<Uuid>, _: &dyn askama::Values, uuid: &Uuid) -> askama::Result<bool> {
	Ok(option_id.as_ref() == Some(uuid))
}
