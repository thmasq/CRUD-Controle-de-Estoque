use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, warn};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TokenInfo {
	pub jti: String,
	pub user_id: Uuid,
	pub expires_at: DateTime<Utc>,
}

/// Thread-safe token blacklist service with comprehensive cleanup
#[derive(Debug, Clone)]
pub struct TokenBlacklistService {
	// Maps token JTI -> TokenInfo for revoked tokens
	revoked_tokens: Arc<DashMap<String, TokenInfo>>,
	// Maps user_id -> Map of JTI -> TokenInfo for active tokens
	active_tokens: Arc<DashMap<Uuid, DashMap<String, TokenInfo>>>,
}

impl TokenBlacklistService {
	#[must_use]
	pub fn new() -> Self {
		Self {
			revoked_tokens: Arc::new(DashMap::new()),
			active_tokens: Arc::new(DashMap::new()),
		}
	}

	/// Register a new active token
	pub fn register_token(&self, token_info: TokenInfo) {
		let user_id = token_info.user_id;
		let jti = token_info.jti.clone();

		// Get or create the user's token map
		let user_tokens = self.active_tokens.entry(user_id).or_default();

		// Store the full token info (including expiration) for cleanup purposes
		user_tokens.insert(jti, token_info);

		debug!("Registered token {} for user {}", user_tokens.len(), user_id);
	}

	/// Check if a token is revoked
	#[must_use]
	pub fn is_token_revoked(&self, jti: &str) -> bool {
		self.revoked_tokens.contains_key(jti)
	}

	/// Revoke a specific token
	#[must_use]
	pub fn revoke_token(&self, jti: &str) -> bool {
		// First, find and remove the token from active tokens
		let mut token_info = None;
		let mut user_id_to_check = None;

		// Search through all users' active tokens
		for user_entry in self.active_tokens.iter() {
			let user_id = *user_entry.key();
			let user_tokens = user_entry.value();

			if let Some((_, info)) = user_tokens.remove(jti) {
				token_info = Some(info);
				user_id_to_check = Some(user_id);
				break;
			}
		}

		// If we found the token, add it to revoked list
		if let Some(mut info) = token_info {
			// Update the expiration to mark when it was revoked
			info.expires_at = Utc::now();

			self.revoked_tokens.insert(jti.to_string(), info);

			// Clean up empty user token maps
			if let Some(user_id) = user_id_to_check
				&& let Some(user_tokens) = self.active_tokens.get(&user_id)
				&& user_tokens.is_empty()
			{
				self.active_tokens.remove(&user_id);
			}

			debug!("Revoked token: {}", jti);
			true
		} else {
			warn!("Attempted to revoke unknown token: {}", jti);
			false
		}
	}

	/// Revoke all tokens for a specific user
	#[must_use]
	pub fn revoke_user_tokens(&self, user_id: Uuid) -> usize {
		let mut revoked_count = 0;

		// Remove all active tokens for this user
		if let Some((_, user_tokens)) = self.active_tokens.remove(&user_id) {
			let now = Utc::now();

			// Move all active tokens to the revoked list
			for (jti, mut token_info) in user_tokens {
				token_info.expires_at = now; // Mark as revoked now
				self.revoked_tokens.insert(jti, token_info);
				revoked_count += 1;
			}
		}

		if revoked_count > 0 {
			debug!("Revoked {} tokens for user {}", revoked_count, user_id);
		}

		revoked_count
	}

	/// Comprehensive cleanup of expired tokens
	/// This should be called periodically to prevent memory leaks
	pub fn cleanup_expired_tokens(&self) {
		let now = Utc::now();
		let mut total_cleaned = 0;

		// Clean up expired revoked tokens
		let revoked_before = self.revoked_tokens.len();
		self.revoked_tokens.retain(|jti, token_info| {
			let should_keep = token_info.expires_at > now;
			if !should_keep {
				debug!("Cleaning up expired revoked token: {}", jti);
			}
			should_keep
		});
		let revoked_cleaned = revoked_before - self.revoked_tokens.len();
		total_cleaned += revoked_cleaned;

		// Clean up expired active tokens
		let mut users_to_remove = Vec::new();
		let mut active_cleaned = 0;

		for user_entry in self.active_tokens.iter() {
			let user_id = *user_entry.key();
			let user_tokens = user_entry.value();

			// Remove expired tokens for this user
			let before_count = user_tokens.len();
			user_tokens.retain(|jti, token_info| {
				let should_keep = token_info.expires_at > now;
				if !should_keep {
					debug!("Cleaning up expired active token: {} for user: {}", jti, user_id);
				}
				should_keep
			});
			let user_cleaned = before_count - user_tokens.len();
			active_cleaned += user_cleaned;

			// Mark user for removal if they have no tokens left
			if user_tokens.is_empty() {
				users_to_remove.push(user_id);
			}
		}

		// Remove users with no active tokens
		for user_id in users_to_remove {
			self.active_tokens.remove(&user_id);
			debug!("Removed empty token map for user: {}", user_id);
		}

		total_cleaned += active_cleaned;

		if total_cleaned > 0 {
			debug!(
				"Token cleanup completed: {} total tokens cleaned ({} revoked, {} active)",
				total_cleaned, revoked_cleaned, active_cleaned
			);
		}
	}

	/// Get comprehensive statistics about the token blacklist
	#[must_use]
	pub fn get_stats(&self) -> BlacklistStats {
		let active_users_count = self.active_tokens.len();
		let total_active_tokens = self.active_tokens.iter().map(|entry| entry.value().len()).sum();

		BlacklistStats {
			revoked_tokens_count: self.revoked_tokens.len(),
			active_users_count,
			total_active_tokens,
		}
	}

	/// Get detailed statistics for debugging
	#[must_use]
	pub fn get_detailed_stats(&self) -> DetailedBlacklistStats {
		let now = Utc::now();
		let mut expired_revoked = 0;
		let mut expired_active = 0;

		// Count expired revoked tokens
		for token_info in self.revoked_tokens.iter() {
			if token_info.value().expires_at <= now {
				expired_revoked += 1;
			}
		}

		// Count expired active tokens
		for user_tokens in self.active_tokens.iter() {
			for token_info in user_tokens.value() {
				if token_info.value().expires_at <= now {
					expired_active += 1;
				}
			}
		}

		let basic_stats = self.get_stats();

		DetailedBlacklistStats {
			basic: basic_stats,
			expired_revoked_tokens: expired_revoked,
			expired_active_tokens: expired_active,
		}
	}

	/// Force cleanup of all tokens for a specific user (useful for testing)
	#[cfg(test)]
	pub fn force_cleanup_user(&self, user_id: Uuid) {
		self.active_tokens.remove(&user_id);

		// Also remove any revoked tokens for this user
		self.revoked_tokens
			.retain(|_, token_info| token_info.user_id != user_id);
	}

	/// Get all active token JTIs for a user (useful for testing)
	#[cfg(test)]
	pub fn get_user_active_tokens(&self, user_id: Uuid) -> Vec<String> {
		self.active_tokens
			.get(&user_id)
			.map(|user_tokens| user_tokens.iter().map(|entry| entry.key().clone()).collect())
			.unwrap_or_default()
	}
}

impl Default for TokenBlacklistService {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug)]
pub struct BlacklistStats {
	pub revoked_tokens_count: usize,
	pub active_users_count: usize,
	pub total_active_tokens: usize,
}

#[derive(Debug)]
pub struct DetailedBlacklistStats {
	pub basic: BlacklistStats,
	pub expired_revoked_tokens: usize,
	pub expired_active_tokens: usize,
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Duration;

	#[test]
	fn test_token_registration_and_revocation() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();
		let jti = "test-token-123".to_string();

		let token_info = TokenInfo {
			jti: jti.clone(),
			user_id,
			expires_at: Utc::now() + Duration::hours(1),
		};

		// Register token
		service.register_token(token_info);

		// Verify token is not revoked
		assert!(!service.is_token_revoked(&jti));

		// Verify token is in active tokens
		let active_tokens = service.get_user_active_tokens(user_id);
		assert_eq!(active_tokens.len(), 1);
		assert_eq!(active_tokens[0], jti);

		// Revoke token
		assert!(service.revoke_token(&jti));

		// Verify token is now revoked
		assert!(service.is_token_revoked(&jti));

		// Verify token is no longer in active tokens
		let active_tokens = service.get_user_active_tokens(user_id);
		assert_eq!(active_tokens.len(), 0);
	}

	#[test]
	fn test_revoke_user_tokens() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();

		// Register multiple tokens for the user
		for i in 0..3 {
			let token_info = TokenInfo {
				jti: format!("token-{}", i),
				user_id,
				expires_at: Utc::now() + Duration::hours(1),
			};
			service.register_token(token_info);
		}

		// Verify all tokens are active
		let active_tokens = service.get_user_active_tokens(user_id);
		assert_eq!(active_tokens.len(), 3);

		// Revoke all user tokens
		let revoked_count = service.revoke_user_tokens(user_id);
		assert_eq!(revoked_count, 3);

		// Verify all tokens are now revoked
		for i in 0..3 {
			assert!(service.is_token_revoked(&format!("token-{}", i)));
		}

		// Verify no active tokens remain
		let active_tokens = service.get_user_active_tokens(user_id);
		assert_eq!(active_tokens.len(), 0);
	}

	#[test]
	fn test_cleanup_expired_tokens() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();

		// Register expired and non-expired tokens
		let expired_token = TokenInfo {
			jti: "expired-token".to_string(),
			user_id,
			expires_at: Utc::now() - Duration::hours(1), // Already expired
		};

		let valid_token = TokenInfo {
			jti: "valid-token".to_string(),
			user_id,
			expires_at: Utc::now() + Duration::hours(1), // Valid for 1 hour
		};

		service.register_token(expired_token);
		service.register_token(valid_token);

		// Verify both tokens are registered
		let active_tokens = service.get_user_active_tokens(user_id);
		assert_eq!(active_tokens.len(), 2);

		// Run cleanup
		service.cleanup_expired_tokens();

		// Verify only the valid token remains
		let active_tokens = service.get_user_active_tokens(user_id);
		assert_eq!(active_tokens.len(), 1);
		assert_eq!(active_tokens[0], "valid-token");
	}

	#[test]
	fn test_cleanup_expired_revoked_tokens() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();

		// Register and revoke a token
		let token_info = TokenInfo {
			jti: "revoked-token".to_string(),
			user_id,
			expires_at: Utc::now() + Duration::hours(1),
		};

		service.register_token(token_info);
		let _ = service.revoke_token("revoked-token");

		// Verify token is revoked
		assert!(service.is_token_revoked("revoked-token"));

		// Manually set the revoked token's expiration to the past
		if let Some(mut revoked_token) = service.revoked_tokens.get_mut("revoked-token") {
			revoked_token.expires_at = Utc::now() - Duration::hours(1);
		}

		// Run cleanup
		service.cleanup_expired_tokens();

		// Verify the expired revoked token was cleaned up
		assert!(!service.is_token_revoked("revoked-token"));
	}

	#[test]
	fn test_stats() {
		let service = TokenBlacklistService::new();
		let user1 = Uuid::new_v4();
		let user2 = Uuid::new_v4();

		// Register tokens for two users
		for i in 0..2 {
			let token_info = TokenInfo {
				jti: format!("user1-token-{}", i),
				user_id: user1,
				expires_at: Utc::now() + Duration::hours(1),
			};
			service.register_token(token_info);
		}

		for i in 0..3 {
			let token_info = TokenInfo {
				jti: format!("user2-token-{}", i),
				user_id: user2,
				expires_at: Utc::now() + Duration::hours(1),
			};
			service.register_token(token_info);
		}

		// Revoke one token
		let _ = service.revoke_token("user1-token-0");

		let stats = service.get_stats();
		assert_eq!(stats.active_users_count, 2);
		assert_eq!(stats.total_active_tokens, 4); // 1 + 3 (one was revoked)
		assert_eq!(stats.revoked_tokens_count, 1);
	}

	#[test]
	fn test_revoke_nonexistent_token() {
		let service = TokenBlacklistService::new();

		// Try to revoke a token that doesn't exist
		assert!(!service.revoke_token("nonexistent-token"));

		// Verify it's not marked as revoked
		assert!(!service.is_token_revoked("nonexistent-token"));
	}

	#[test]
	fn test_user_cleanup_after_all_tokens_expire() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();

		// Register an expired token
		let token_info = TokenInfo {
			jti: "expired-token".to_string(),
			user_id,
			expires_at: Utc::now() - Duration::hours(1),
		};
		service.register_token(token_info);

		// Verify user has tokens
		assert_eq!(service.active_tokens.len(), 1);

		// Run cleanup
		service.cleanup_expired_tokens();

		// Verify user entry was removed completely
		assert_eq!(service.active_tokens.len(), 0);
	}
}
