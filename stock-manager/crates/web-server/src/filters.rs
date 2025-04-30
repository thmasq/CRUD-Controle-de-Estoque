use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[allow(clippy::unnecessary_wraps)]
pub fn date(value: &DateTime<Utc>, _: &dyn askama::Values) -> askama::Result<String> {
	Ok(value.format("%Y-%m-%d %H:%M").to_string())
}

#[allow(clippy::unnecessary_wraps)]
pub fn default_string(value: &str, _: &dyn askama::Values, default_value: &str) -> askama::Result<String> {
	if value.is_empty() {
		Ok(default_value.to_string())
	} else {
		Ok(value.to_string())
	}
}

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::ref_option)]
pub fn default_option(value: &Option<String>, _: &dyn askama::Values, default_value: &str) -> askama::Result<String> {
	match value {
		Some(s) if !s.is_empty() => Ok(s.clone()),
		_ => Ok(default_value.to_string()),
	}
}

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn default_int(value: &i32, _: &dyn askama::Values, default_value: &i32) -> askama::Result<String> {
	if *value == 0 {
		Ok(default_value.to_string())
	} else {
		Ok(value.to_string())
	}
}

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn default_decimal(value: &Decimal, _: &dyn askama::Values, default_value: &f64) -> askama::Result<String> {
	if value.is_zero() {
		Ok(format!("{default_value:.2}"))
	} else {
		Ok(format!("{value:.2}"))
	}
}

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::ref_option)]
pub fn option_uuid_eq(option_id: &Option<Uuid>, _: &dyn askama::Values, uuid: &Uuid) -> askama::Result<bool> {
	Ok(option_id.as_ref() == Some(uuid))
}

#[allow(clippy::unnecessary_wraps)]
pub fn format_decimal(value: &Decimal, _: &dyn askama::Values) -> askama::Result<String> {
	Ok(format!("{value:.2}"))
}
