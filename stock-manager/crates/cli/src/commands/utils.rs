use anyhow::{Result, anyhow};
use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Password};
use serde_json;
use stock_domain::entities::user::{User, UserRole};
use uuid::Uuid;

use crate::CliContext;

pub fn parse_user_role(role_str: &str) -> Result<UserRole> {
	match role_str.to_uppercase().as_str() {
		"SELLER" => Ok(UserRole::Seller),
		"MANAGER" => Ok(UserRole::Manager),
		_ => Err(anyhow!("Invalid role. Must be SELLER or MANAGER")),
	}
}

pub async fn find_user_by_identifier(ctx: &CliContext, identifier: &str) -> Result<Option<User>> {
	// Try to parse as UUID first
	if let Ok(uuid) = Uuid::parse_str(identifier) {
		return ctx.auth_service.user_repository.find_by_id(uuid).await;
	}

	// If not UUID, treat as username
	ctx.auth_service.user_repository.find_by_username(identifier).await
}

pub fn confirm_action(message: &str, skip_confirmation: bool) -> Result<bool> {
	if skip_confirmation {
		return Ok(true);
	}

	Ok(Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt(message)
		.default(false)
		.interact()?)
}

pub fn prompt_password(prompt: &str) -> Result<String> {
	let password = Password::with_theme(&ColorfulTheme::default())
		.with_prompt(prompt)
		.with_confirmation("Confirm password", "Passwords don't match")
		.interact()?;

	if password.is_empty() {
		return Err(anyhow!("Password cannot be empty"));
	}

	if password.len() < 6 {
		return Err(anyhow!("Password must be at least 6 characters long"));
	}

	Ok(password)
}

pub fn format_user_output(users: &[User], format: &str) -> Result<String> {
	match format.to_lowercase().as_str() {
		"json" => {
			let output = serde_json::to_string_pretty(users)?;
			Ok(output)
		},
		"table" => {
			let mut table = Table::new();
			table
				.load_preset(UTF8_FULL)
				.apply_modifier(UTF8_ROUND_CORNERS)
				.set_header(vec!["ID", "Username", "Role", "Created At", "Updated At"]);

			for user in users {
				table.add_row(vec![
					user.id.to_string(),
					user.username.clone(),
					user.role.to_string(),
					user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
					user.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
				]);
			}

			Ok(table.to_string())
		},
		_ => Err(anyhow!("Unsupported format: {}", format)),
	}
}

pub fn format_single_user_output(user: &User, format: &str) -> Result<String> {
	match format.to_lowercase().as_str() {
		"json" => {
			let output = serde_json::to_string_pretty(user)?;
			Ok(output)
		},
		"table" => {
			let mut table = Table::new();
			table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);

			table.add_row(vec!["Field", "Value"]);
			table.add_row(vec!["ID", &user.id.to_string()]);
			table.add_row(vec!["Username", &user.username]);
			table.add_row(vec!["Role", &user.role.to_string()]);
			table.add_row(vec![
				"Created At",
				&user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
			]);
			table.add_row(vec![
				"Updated At",
				&user.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
			]);

			Ok(table.to_string())
		},
		_ => Err(anyhow!("Unsupported format: {}", format)),
	}
}

pub fn print_success(message: &str) {
	println!("✅ {message}");
}

pub fn print_error(message: &str) {
	eprintln!("❌ Error: {message}");
}

pub fn print_warning(message: &str) {
	println!("⚠️  Warning: {message}");
}

pub fn print_info(message: &str) {
	println!("ℹ️  {message}");
}
