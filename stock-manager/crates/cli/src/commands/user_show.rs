use anyhow::{Result, anyhow};

use crate::CliContext;
use crate::utils::*;

pub async fn execute(ctx: &CliContext, identifier: &str, format: &str) -> Result<()> {
	print_info(&format!("Looking up user '{}'...", identifier));

	let user = find_user_by_identifier(ctx, identifier).await?;

	match user {
		Some(user) => {
			let output = format_single_user_output(&user, format)?;
			println!("{}", output);
			Ok(())
		},
		None => {
			print_error(&format!("User '{}' not found", identifier));
			Err(anyhow!("User not found"))
		},
	}
}
