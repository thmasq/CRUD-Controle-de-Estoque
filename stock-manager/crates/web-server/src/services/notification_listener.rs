use futures_channel::mpsc;
use futures_util::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio_postgres::{AsyncMessage, NoTls, connect};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::token_blacklist::TokenBlacklistService;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UserDeletedPayload {
	user_id: String,
	username: String,
	deleted_at: String,
}

pub struct NotificationListener {
	blacklist_service: Arc<TokenBlacklistService>,
	database_url: String,
}

impl NotificationListener {
	#[must_use]
	pub const fn new(blacklist_service: Arc<TokenBlacklistService>, database_url: String) -> Self {
		Self {
			blacklist_service,
			database_url,
		}
	}

	pub async fn start(&self) -> anyhow::Result<()> {
		info!("Starting notification listener...");

		loop {
			match self.listen_loop().await {
				Ok(()) => {
					info!("Notification listener loop ended gracefully");
					break;
				},
				Err(e) => {
					error!("Notification listener error: {}", e);
					warn!("Reconnecting in 5 seconds...");
					tokio::time::sleep(Duration::from_secs(5)).await;
				},
			}
		}
		Ok(())
	}

	async fn listen_loop(&self) -> anyhow::Result<()> {
		info!("Connecting to PostgreSQL for notifications...");

		let (client, connection) = connect(&self.database_url, NoTls).await?;
		info!("Connected to PostgreSQL, starting to listen for notifications");

		let (tx, mut rx) = mpsc::unbounded();

		let connection_handle = tokio::spawn(async move {
			tokio::pin!(connection);

			info!("Starting connection driver with notification interception...");

			loop {
				match futures_util::future::poll_fn(|cx| connection.as_mut().poll_message(cx)).await {
					Some(Ok(AsyncMessage::Notification(notification))) => {
						debug!(
							"Intercepted notification on channel '{}': {}",
							notification.channel(),
							notification.payload()
						);

						if tx.unbounded_send(notification).is_err() {
							error!("Notification channel closed");
							break;
						}
					},
					Some(Ok(AsyncMessage::Notice(notice))) => {
						debug!("PostgreSQL notice: {}", notice.message());
					},
					Some(Err(e)) => {
						error!("PostgreSQL connection error: {}", e);
						break;
					},
					None => {
						info!("PostgreSQL connection closed");
						break;
					},
					Some(Ok(_)) => todo!(),
				}
			}
			info!("Connection driver terminated");
		});

		debug!("Executing LISTEN command...");
		client.batch_execute("LISTEN user_deleted;").await?;
		info!("Now listening to 'user_deleted' channel");

		let blacklist_service = self.blacklist_service.clone();
		let notification_handle = tokio::spawn(async move {
			let mut count = 0;

			while let Some(notification) = rx.next().await {
				count += 1;
				debug!(
					"Processing notification #{}: channel='{}', payload='{}'",
					count,
					notification.channel(),
					notification.payload()
				);

				if notification.channel() == "user_deleted" {
					if let Err(e) =
						Self::handle_user_deleted_notification(&blacklist_service, notification.payload()).await
					{
						error!("Error handling user_deleted notification: {}", e);
					} else {
						info!("Successfully processed user deletion notification");
					}
				}
			}
			info!("Notification handler terminated after {} notifications", count);
		});

		let cleanup_blacklist = self.blacklist_service.clone();
		let cleanup_handle = tokio::spawn(async move {
			let mut interval = tokio::time::interval(Duration::from_secs(300));
			loop {
				interval.tick().await;

				let stats_before = cleanup_blacklist.get_detailed_stats();
				cleanup_blacklist.cleanup_expired_tokens();
				let stats_after = cleanup_blacklist.get_stats();

				debug!(
					"Token cleanup completed - Before: {} active, {} revoked ({} expired active, {} expired revoked), After: {} active, {} revoked",
					stats_before.basic.total_active_tokens,
					stats_before.basic.revoked_tokens_count,
					stats_before.expired_active_tokens,
					stats_before.expired_revoked_tokens,
					stats_after.total_active_tokens,
					stats_after.revoked_tokens_count
				);
			}
		});

		tokio::time::sleep(Duration::from_millis(100)).await;

		tokio::select! {
			result = connection_handle => {
				match result {
					Ok(()) => info!("PostgreSQL connection closed"),
					Err(e) => error!("Connection task error: {}", e),
				}
			}
			result = notification_handle => {
				match result {
					Ok(()) => info!("Notification handler closed"),
					Err(e) => error!("Notification handler error: {}", e),
				}
			}
			result = cleanup_handle => {
				match result {
					Ok(_) => info!("Cleanup task closed"),
					Err(e) => error!("Cleanup task error: {}", e),
				}
			}
		}

		Ok(())
	}

	async fn handle_user_deleted_notification(
		blacklist_service: &TokenBlacklistService,
		payload: &str,
	) -> anyhow::Result<()> {
		let user_deleted: UserDeletedPayload = serde_json::from_str(payload)
			.map_err(|e| anyhow::anyhow!("Failed to parse user_deleted payload: {}", e))?;

		let user_id =
			Uuid::parse_str(&user_deleted.user_id).map_err(|e| anyhow::anyhow!("Invalid user_id in payload: {}", e))?;

		info!(
			"User '{}' (ID: {}) was deleted, revoking all tokens",
			user_deleted.username, user_id
		);

		let revoked_count = blacklist_service.revoke_user_tokens(user_id);

		info!(
			"Successfully revoked {} tokens for deleted user '{}' (ID: {})",
			revoked_count, user_deleted.username, user_id
		);

		let stats = blacklist_service.get_stats();
		debug!(
			"Token blacklist stats after user deletion - Active: {}, Revoked: {}",
			stats.total_active_tokens, stats.revoked_tokens_count
		);

		Ok(())
	}
}
