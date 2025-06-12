use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TokenInfo {
	pub jti: String,
	pub user_id: Uuid,
	pub expires_at: DateTime<Utc>,
}

/// Thread-safe token blacklist service
#[derive(Debug, Clone)]
pub struct TokenBlacklistService {
	// Maps token JTI -> TokenInfo for revoked tokens
	revoked_tokens: Arc<DashMap<String, TokenInfo>>,
	// Maps user_id -> set of active token JTIs for that user
	user_tokens: Arc<DashMap<Uuid, HashSet<String>>>,
}

impl TokenBlacklistService {
	#[must_use]
	pub fn new() -> Self {
		Self {
			revoked_tokens: Arc::new(DashMap::new()),
			user_tokens: Arc::new(DashMap::new()),
		}
	}

	pub fn register_token(&self, token_info: TokenInfo) {
		let user_id = token_info.user_id;
		let jti = token_info.jti;

		self.user_tokens.entry(user_id).or_default().insert(jti);
	}

	#[must_use]
	pub fn is_token_revoked(&self, jti: &str) -> bool {
		self.revoked_tokens.contains_key(jti)
	}

	#[must_use]
	pub fn revoke_token(&self, jti: &str) -> bool {
		self.user_tokens
			.iter()
			.find_map(|entry| {
				if entry.value().contains(jti) {
					Some(*entry.key())
				} else {
					None
				}
			})
			.is_some_and(|user_tokens_ref| {
				if let Some(mut user_tokens) = self.user_tokens.get_mut(&user_tokens_ref) {
					user_tokens.remove(jti);
				}

				self.revoked_tokens.insert(
					jti.to_string(),
					TokenInfo {
						jti: jti.to_string(),
						user_id: user_tokens_ref,
						expires_at: Utc::now(),
					},
				);
				true
			})
	}

	#[must_use]
	pub fn revoke_user_tokens(&self, user_id: Uuid) -> usize {
		let mut revoked_count = 0;

		if let Some((_, tokens)) = self.user_tokens.remove(&user_id) {
			for jti in tokens {
				self.revoked_tokens.insert(
					jti.clone(),
					TokenInfo {
						jti,
						user_id,
						expires_at: Utc::now(),
					},
				);
				revoked_count += 1;
			}
		}

		revoked_count
	}

	/// Clean up expired tokens from the blacklist
	/// This should be called periodically to prevent memory leaks
	pub fn cleanup_expired_tokens(&self) {
		let now = Utc::now();

		// Remove expired revoked tokens
		self.revoked_tokens.retain(|_, token_info| token_info.expires_at > now);

		// Note: Active user tokens are cleaned up when they're revoked or when users are deleted
		// We don't need to clean them up based on expiration since they're needed for revocation
	}

	#[must_use]
	pub fn get_stats(&self) -> BlacklistStats {
		BlacklistStats {
			revoked_tokens_count: self.revoked_tokens.len(),
			active_users_count: self.user_tokens.len(),
			total_active_tokens: self.user_tokens.iter().map(|entry| entry.value().len()).sum(),
		}
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_token_registration_and_revocation() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();
		let jti = "test-token-123".to_string();

		let token_info = TokenInfo {
			jti: jti.clone(),
			user_id,
			expires_at: Utc::now() + chrono::Duration::hours(1),
		};

		service.register_token(token_info);
		assert!(!service.is_token_revoked(&jti));

		assert!(service.revoke_token(&jti));
		assert!(service.is_token_revoked(&jti));
	}

	#[test]
	fn test_revoke_user_tokens() {
		let service = TokenBlacklistService::new();
		let user_id = Uuid::new_v4();

		for i in 0..3 {
			let token_info = TokenInfo {
				jti: format!("token-{}", i),
				user_id,
				expires_at: Utc::now() + chrono::Duration::hours(1),
			};
			service.register_token(token_info);
		}

		let revoked_count = service.revoke_user_tokens(user_id);
		assert_eq!(revoked_count, 3);

		for i in 0..3 {
			assert!(service.is_token_revoked(&format!("token-{}", i)));
		}
	}
}
