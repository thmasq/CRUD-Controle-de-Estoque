use anyhow::{Result, anyhow};

use crate::CliContext;
use crate::utils::*;

pub async fn execute(ctx: &CliContext, identifier: &str, skip_confirmation: bool) -> Result<()> {
	print_info(&format!("Looking up user '{}'...", identifier));

	let user = find_user_by_identifier(ctx, identifier)
		.await?
		.ok_or_else(|| anyhow!("User '{}' not found", identifier))?;

	println!("\nUser to be deleted:");
	let output = format_single_user_output(&user, "table")?;
	println!("{}", output);

	// Confirm deletion
	let confirmation_message = format!(
		"Are you sure you want to delete user '{}'? This action cannot be undone!",
		user.username
	);

	if !confirm_action(&confirmation_message, skip_confirmation)? {
		print_info("Operation cancelled");
		return Ok(());
	}

	print_info("Deleting user...");

	// Delete the user
	let deleted = ctx.auth_service.user_repository.delete(user.id).await?;

	if deleted {
		print_success(&format!("User '{}' deleted successfully", user.username));
	} else {
		return Err(anyhow!("Failed to delete user"));
	}

	Ok(())
}
