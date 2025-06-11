use anyhow::Result;

use crate::CliContext;
use crate::utils::*;

pub async fn execute(ctx: &CliContext, format: &str) -> Result<()> {
	print_info("Fetching users...");

	let users = ctx.auth_service.user_repository.find_all().await?;

	if users.is_empty() {
		print_warning("No users found in the system");
		return Ok(());
	}

	let output = format_user_output(&users, format)?;
	println!("{}", output);

	print_success(&format!("Found {} user(s)", users.len()));

	Ok(())
}
