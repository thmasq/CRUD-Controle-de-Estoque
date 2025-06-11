use anyhow::{Result, anyhow};
use chrono::Utc;

use crate::CliContext;
use crate::utils::*;

pub async fn execute(
	ctx: &CliContext,
	identifier: &str,
	new_password: Option<String>,
	skip_confirmation: bool,
) -> Result<()> {
	print_info(&format!("Looking up user '{}'...", identifier));

	let mut user = find_user_by_identifier(ctx, identifier)
		.await?
		.ok_or_else(|| anyhow!("User '{}' not found", identifier))?;

	println!("\nUser:");
	println!("  ID: {}", user.id);
	println!("  Username: {}", user.username);
	println!("  Role: {}", user.role.to_string());

	// Get the new password
	let password = match new_password {
		Some(pwd) => {
			if pwd.is_empty() {
				return Err(anyhow!("Password cannot be empty"));
			}
			if pwd.len() < 6 {
				return Err(anyhow!("Password must be at least 6 characters long"));
			}
			pwd
		},
		None => prompt_password("Enter new password")?,
	};

	// Confirm action
	let confirmation_message = format!("Change password for user '{}'?", user.username);

	if !confirm_action(&confirmation_message, skip_confirmation)? {
		print_info("Operation cancelled");
		return Ok(());
	}

	print_info("Updating password...");

	// Hash the new password
	let hashed_password = ctx.auth_service.user_repository.hash_password(&password).await?;

	// Update user with new password
	user.password_hash = hashed_password;
	user.updated_at = Utc::now();

	// Save the user
	let updated_user = ctx.auth_service.user_repository.update(user).await?;

	print_success(&format!(
		"Password updated successfully for user '{}'",
		updated_user.username
	));

	Ok(())
}
