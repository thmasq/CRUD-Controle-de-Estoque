use futures_channel::mpsc;
use futures_util::{StreamExt, stream};
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

		let (client, mut connection) = connect(&self.database_url, NoTls).await?;

		info!("Connected to PostgreSQL, starting to listen for notifications");

		client.batch_execute("LISTEN user_deleted;").await?;

		info!("Now listening to 'user_deleted' channel");

		let (tx, mut rx) = mpsc::unbounded();

		let connection_handle = tokio::spawn(async move {
			let mut message_stream = stream::poll_fn(move |cx| connection.poll_message(cx));

			while let Some(res) = message_stream.next().await {
				match res {
					Ok(msg) => {
						if let Err(send_err) = tx.unbounded_send(msg) {
							error!("Channel send error: {}", send_err);
							break;
						}
					},
					Err(db_err) => {
						error!("Postgres connection error: {}", db_err);
						break;
					},
				}
			}
			debug!("Database connection handler terminated");
		});

		let blacklist_service = self.blacklist_service.clone();
		let notification_handle = tokio::spawn(async move {
			while let Some(message) = rx.next().await {
				match message {
					AsyncMessage::Notification(notification) => {
						debug!(
							"Received notification on channel '{}': {}",
							notification.channel(),
							notification.payload()
						);

						if notification.channel() == "user_deleted"
							&& let Err(e) =
								Self::handle_user_deleted_notification(&blacklist_service, notification.payload()).await
						{
							error!("Error handling user_deleted notification: {}", e);
						}
					},
					AsyncMessage::Notice(notice) => {
						debug!("Received PostgreSQL notice: {}", notice.message());
					},
					_ => {
						debug!("Received other PostgreSQL message");
					},
				}
			}
			debug!("Notification handler terminated");
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
			"Token blacklist stats after user deletion - Active users: {}, Total active tokens: {}, Revoked tokens: {}",
			stats.active_users_count, stats.total_active_tokens, stats.revoked_tokens_count
		);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::services::token_blacklist::TokenInfo;
	use chrono::{Duration, Utc};

	#[tokio::test]
	async fn test_user_deleted_payload_parsing() {
		let payload = r#"{"user_id":"550e8400-e29b-41d4-a716-446655440000","username":"testuser","deleted_at":"2024-01-01T12:00:00Z"}"#;

		let parsed: UserDeletedPayload = serde_json::from_str(payload).unwrap();
		assert_eq!(parsed.user_id, "550e8400-e29b-41d4-a716-446655440000");
		assert_eq!(parsed.username, "testuser");
	}

	#[tokio::test]
	async fn test_handle_user_deleted_notification() {
		let blacklist_service = Arc::new(TokenBlacklistService::new());
		let user_id = Uuid::new_v4();

		// Register some tokens for the user
		for i in 0..3 {
			blacklist_service.register_token(TokenInfo {
				jti: format!("token-{}", i),
				user_id,
				expires_at: Utc::now() + Duration::hours(1),
			});
		}

		// Verify tokens are active
		let stats_before = blacklist_service.get_stats();
		assert_eq!(stats_before.total_active_tokens, 3);
		assert_eq!(stats_before.revoked_tokens_count, 0);

		// Simulate user deletion notification
		let payload = format!(
			r#"{{"user_id":"{}","username":"testuser","deleted_at":"2024-01-01T12:00:00Z"}}"#,
			user_id
		);

		NotificationListener::handle_user_deleted_notification(&blacklist_service, &payload)
			.await
			.unwrap();

		// Verify all tokens were revoked
		let stats_after = blacklist_service.get_stats();
		assert_eq!(stats_after.total_active_tokens, 0);
		assert_eq!(stats_after.revoked_tokens_count, 3);

		// Verify individual tokens are marked as revoked
		for i in 0..3 {
			assert!(blacklist_service.is_token_revoked(&format!("token-{}", i)));
		}
	}

	#[tokio::test]
	async fn test_handle_invalid_payload() {
		let blacklist_service = Arc::new(TokenBlacklistService::new());

		// Test with invalid JSON
		let result = NotificationListener::handle_user_deleted_notification(&blacklist_service, "invalid json").await;
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("Failed to parse"));

		// Test with invalid UUID
		let payload = r#"{"user_id":"invalid-uuid","username":"testuser","deleted_at":"2024-01-01T12:00:00Z"}"#;
		let result = NotificationListener::handle_user_deleted_notification(&blacklist_service, payload).await;
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("Invalid user_id"));
	}

	#[tokio::test]
	async fn test_user_deleted_notification_with_no_tokens() {
		let blacklist_service = Arc::new(TokenBlacklistService::new());
		let user_id = Uuid::new_v4();

		let payload = format!(
			r#"{{"user_id":"{}","username":"testuser","deleted_at":"2024-01-01T12:00:00Z"}}"#,
			user_id
		);

		// Should not fail even if user has no tokens
		let result = NotificationListener::handle_user_deleted_notification(&blacklist_service, &payload).await;
		assert!(result.is_ok());

		// Should report 0 tokens revoked
		let stats = blacklist_service.get_stats();
		assert_eq!(stats.total_active_tokens, 0);
		assert_eq!(stats.revoked_tokens_count, 0);
	}
}
