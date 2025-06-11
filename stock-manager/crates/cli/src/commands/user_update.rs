use anyhow::{Result, anyhow};
use chrono::Utc;

use crate::CliContext;
use crate::utils::*;

pub async fn execute(
	ctx: &CliContext,
	identifier: &str,
	new_username: Option<String>,
	new_password: Option<String>,
	new_role: Option<String>,
	skip_confirmation: bool,
) -> Result<()> {
	print_info(&format!("Looking up user '{}'...", identifier));

	let user = find_user_by_identifier(ctx, identifier)
		.await?
		.ok_or_else(|| anyhow!("User '{}' not found", identifier))?;

	let mut changes = Vec::new();
	let mut updated_user = user.clone();

	// Check if new username is provided and different
	if let Some(ref username) = new_username {
		if username.is_empty() {
			return Err(anyhow!("Username cannot be empty"));
		}
		if username.len() < 3 {
			return Err(anyhow!("Username must be at least 3 characters long"));
		}
		if username != &user.username {
			// Check if username already exists
			if let Some(_) = ctx.auth_service.user_repository.find_by_username(username).await? {
				return Err(anyhow!("Username '{}' already exists", username));
			}
			changes.push(format!("username: '{}' -> '{}'", user.username, username));
			updated_user.username = username.clone();
		}
	}

	// Check if new role is provided and different
	if let Some(ref role_str) = new_role {
		let role = parse_user_role(role_str)?;
		if role != user.role {
			changes.push(format!("role: '{}' -> '{}'", user.role.to_string(), role.to_string()));
			updated_user.role = role;
		}
	}

	// Handle password update
	let _ = if new_password.is_some()
		|| dialoguer::Confirm::new()
			.with_prompt("Do you want to change the password?")
			.default(false)
			.interact()?
	{
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

		let hashed_password = ctx.auth_service.user_repository.hash_password(&password).await?;
		updated_user.password_hash = hashed_password;
		changes.push("password: updated".to_string());
		true
	} else {
		false
	};

	if changes.is_empty() {
		print_info("No changes to make");
		return Ok(());
	}

	// Show changes
	println!("\nChanges to be made:");
	for change in &changes {
		println!("  • {}", change);
	}

	// Confirm action
	if !confirm_action("Apply these changes?", skip_confirmation)? {
		print_info("Operation cancelled");
		return Ok(());
	}

	print_info("Updating user...");

	// Update timestamp
	updated_user.updated_at = Utc::now();

	// Save the user
	let updated_user = ctx.auth_service.user_repository.update(updated_user).await?;

	print_success(&format!("User '{}' updated successfully", updated_user.username));

	// Display updated user details
	println!("\nUpdated User Details:");
	let output = format_single_user_output(&updated_user, "table")?;
	println!("{}", output);

	Ok(())
}
