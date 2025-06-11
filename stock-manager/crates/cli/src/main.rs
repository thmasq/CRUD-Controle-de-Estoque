use anyhow::Result;
use clap::{Parser, Subcommand};
use stock_cli::{CliContext, user_create, user_delete, user_list, user_password, user_show, user_update};

#[derive(Parser)]
#[command(name = "stock-cli")]
#[command(about = "Stock Manager CLI - User Management Tool")]
#[command(version = "0.1.0")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// List all users in the system
	List {
		/// Output format (table, json)
		#[arg(short, long, default_value = "table")]
		format: String,
	},
	/// Create a new user
	Create {
		/// Username for the new user
		#[arg(short, long)]
		username: String,
		/// Password for the new user (will prompt if not provided)
		#[arg(short, long)]
		password: Option<String>,
		/// User role (SELLER or MANAGER)
		#[arg(short, long, default_value = "SELLER")]
		role: String,
		/// Skip confirmation prompt
		#[arg(short = 'y', long)]
		yes: bool,
	},
	/// Show details of a specific user
	Show {
		/// User identifier (UUID or username)
		identifier: String,
		/// Output format (table, json)
		#[arg(short, long, default_value = "table")]
		format: String,
	},
	/// Update an existing user
	Update {
		/// User identifier (UUID or username)
		identifier: String,
		/// New username
		#[arg(long)]
		username: Option<String>,
		/// New password (will prompt if not provided)
		#[arg(long)]
		password: Option<String>,
		/// New role (SELLER or MANAGER)
		#[arg(long)]
		role: Option<String>,
		/// Skip confirmation prompt
		#[arg(short = 'y', long)]
		yes: bool,
	},
	/// Delete a user
	Delete {
		/// User identifier (UUID or username)
		identifier: String,
		/// Skip confirmation prompt
		#[arg(short = 'y', long)]
		yes: bool,
	},
	/// Change user password
	Password {
		/// User identifier (UUID or username)
		identifier: String,
		/// New password (will prompt if not provided)
		#[arg(short, long)]
		password: Option<String>,
		/// Skip confirmation prompt
		#[arg(short = 'y', long)]
		yes: bool,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let ctx = CliContext::new()?;

	match cli.command {
		Commands::List { format } => {
			user_list::execute(&ctx, &format).await?;
		},
		Commands::Create {
			username,
			password,
			role,
			yes,
		} => {
			user_create::execute(&ctx, &username, password, &role, yes).await?;
		},
		Commands::Show { identifier, format } => {
			user_show::execute(&ctx, &identifier, &format).await?;
		},
		Commands::Update {
			identifier,
			username,
			password,
			role,
			yes,
		} => {
			user_update::execute(&ctx, &identifier, username, password, role, yes).await?;
		},
		Commands::Delete { identifier, yes } => {
			user_delete::execute(&ctx, &identifier, yes).await?;
		},
		Commands::Password {
			identifier,
			password,
			yes,
		} => {
			user_password::execute(&ctx, &identifier, password, yes).await?;
		},
	}

	Ok(())
}
