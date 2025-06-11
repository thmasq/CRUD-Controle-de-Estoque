use anyhow::{Result, anyhow};

use crate::CliContext;
use crate::utils::{find_user_by_identifier, format_single_user_output, print_error, print_info};

pub async fn execute(ctx: &CliContext, identifier: &str, format: &str) -> Result<()> {
	print_info(&format!("Looking up user '{identifier}'..."));

	let user = find_user_by_identifier(ctx, identifier).await?;

	if let Some(user) = user {
		let output = format_single_user_output(&user, format)?;
		println!("{output}");
		Ok(())
	} else {
		print_error(&format!("User '{identifier}' not found"));
		Err(anyhow!("User not found"))
	}
}
