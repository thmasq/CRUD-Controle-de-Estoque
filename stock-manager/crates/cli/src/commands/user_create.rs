use anyhow::{Result, anyhow};
use stock_application::services::auth_service::RegisterUserDto;

use crate::CliContext;
use crate::utils::{
	confirm_action, format_single_user_output, parse_user_role, print_info, print_success, prompt_password,
};

pub async fn execute(
	ctx: &CliContext,
	username: &str,
	password: Option<String>,
	role: &str,
	skip_confirmation: bool,
) -> Result<()> {
	// Validate username
	if username.is_empty() {
		return Err(anyhow!("Username cannot be empty"));
	}

	if username.len() < 3 {
		return Err(anyhow!("Username must be at least 3 characters long"));
	}

	// Check if user already exists
	if (ctx.auth_service.user_repository.find_by_username(username).await?).is_some() {
		return Err(anyhow!("User '{}' already exists", username));
	}

	// Parse role
	let user_role = parse_user_role(role)?;

	// Get password
	let password = match password {
		Some(pwd) => {
			if pwd.is_empty() {
				return Err(anyhow!("Password cannot be empty"));
			}
			if pwd.len() < 6 {
				return Err(anyhow!("Password must be at least 6 characters long"));
			}
			pwd
		},
		None => prompt_password("Enter password for new user")?,
	};

	// Confirm action
	let confirmation_message = format!("Create user '{username}' with role '{user_role}'?");

	if !confirm_action(&confirmation_message, skip_confirmation)? {
		print_info("Operation cancelled");
		return Ok(());
	}

	print_info("Creating user...");

	// Create the user
	let register_dto = RegisterUserDto {
		username: username.to_string(),
		password,
		role: user_role,
	};

	let created_user = ctx.auth_service.register(register_dto).await?;

	print_success(&format!(
		"User '{}' created successfully with ID: {}",
		created_user.username, created_user.id
	));

	// Display created user details
	println!("\nUser Details:");
	let output = format_single_user_output(&created_user, "table")?;
	println!("{output}");

	Ok(())
}
