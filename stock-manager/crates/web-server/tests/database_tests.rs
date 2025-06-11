use std::sync::Arc;
use stock_application::services::auth_service::{AuthService, Credentials, RegisterUserDto};
use stock_domain::entities::user::UserRole;
use stock_domain::repositories::user_repository::UserRepository;
use stock_infrastructure::db::establish_connection_pool;
use stock_infrastructure::repositories::user_repository::DieselUserRepository;

#[tokio::test]
async fn test_user_operations() {
	let pool = establish_connection_pool();
	let pool = Arc::new(pool);
	let user_repo = Arc::new(DieselUserRepository::new(pool));
	let auth_service = AuthService::new(user_repo, "test_secret".to_string());

	let register_dto = RegisterUserDto {
		username: "testdbuser".to_string(),
		password: "testpassword".to_string(),
		role: UserRole::Manager,
	};

	let result = auth_service.register(register_dto).await;
	match result {
		Ok(user) => {
			assert_eq!(user.username, "testdbuser");
			assert_eq!(user.role, UserRole::Manager);
		},
		Err(e) => {
			// If the error is about user already existing, that's fine for tests
			if !e.to_string().contains("already exists") {
				panic!("Unexpected error during registration: {}", e);
			}
		},
	}

	// Test user login
	let credentials = Credentials {
		username: "testdbuser".to_string(),
		password: "testpassword".to_string(),
	};

	let login_result = auth_service.login(credentials).await;
	assert!(login_result.is_ok(), "Login should succeed");

	let auth_token = login_result.unwrap();
	assert!(!auth_token.token.is_empty());
	assert_eq!(auth_token.role, UserRole::Manager);
}

#[tokio::test]
async fn test_invalid_login() {
	let pool = establish_connection_pool();
	let pool = Arc::new(pool);
	let user_repo = Arc::new(DieselUserRepository::new(pool));
	let auth_service = AuthService::new(user_repo, "test_secret".to_string());

	// Test login with invalid credentials
	let credentials = Credentials {
		username: "nonexistentuser".to_string(),
		password: "wrongpassword".to_string(),
	};

	let login_result = auth_service.login(credentials).await;
	assert!(login_result.is_err(), "Login should fail for invalid credentials");
}

#[tokio::test]
async fn test_password_hashing() {
	let pool = establish_connection_pool();
	let pool = Arc::new(pool);
	let user_repo = Arc::new(DieselUserRepository::new(pool));

	let password = "test_password_123";

	// Test password hashing
	let hash_result = user_repo.hash_password(password).await;
	assert!(hash_result.is_ok(), "Password hashing should succeed");

	let password_hash = hash_result.unwrap();
	assert!(!password_hash.is_empty());
	assert_ne!(password_hash, password); // Hash should be different from original

	// Test password verification
	let verify_result = user_repo.verify_password(password, &password_hash).await;
	assert!(verify_result.is_ok(), "Password verification should succeed");
	assert!(verify_result.unwrap(), "Password should be verified as correct");

	// Test verification with wrong password
	let wrong_verify_result = user_repo.verify_password("wrong_password", &password_hash).await;
	assert!(wrong_verify_result.is_ok(), "Password verification should complete");
	assert!(!wrong_verify_result.unwrap(), "Wrong password should not be verified");
}

#[tokio::test]
async fn test_duplicate_username() {
	let pool = establish_connection_pool();
	let pool = Arc::new(pool);
	let user_repo = Arc::new(DieselUserRepository::new(pool));
	let auth_service = AuthService::new(user_repo, "test_secret".to_string());

	let username = "duplicate_test_user";

	// First registration should succeed
	let register_dto1 = RegisterUserDto {
		username: username.to_string(),
		password: "password1".to_string(),
		role: UserRole::Seller,
	};

	let _first_result = auth_service.register(register_dto1).await;
	// This might succeed or fail if the user already exists from previous tests

	// Second registration with same username should fail
	let register_dto2 = RegisterUserDto {
		username: username.to_string(),
		password: "password2".to_string(),
		role: UserRole::Manager,
	};

	let second_result = auth_service.register(register_dto2).await;
	assert!(
		second_result.is_err(),
		"Second registration with same username should fail"
	);
	assert!(second_result.unwrap_err().to_string().contains("already exists"));
}

#[tokio::test]
async fn test_jwt_token_validation() {
	let pool = establish_connection_pool();
	let pool = Arc::new(pool);
	let user_repo = Arc::new(DieselUserRepository::new(pool));
	let auth_service = AuthService::new(user_repo, "test_secret_key".to_string());

	// Register a user
	let register_dto = RegisterUserDto {
		username: "jwt_test_user".to_string(),
		password: "test_password".to_string(),
		role: UserRole::Manager,
	};

	let _user = match auth_service.register(register_dto).await {
		Ok(user) => user,
		Err(e) if e.to_string().contains("already exists") => {
			// User already exists, continue with login
			auth_service
				.user_repository
				.find_by_username("jwt_test_user")
				.await
				.unwrap()
				.unwrap()
		},
		Err(e) => panic!("Unexpected error: {}", e),
	};

	// Login to get a token
	let credentials = Credentials {
		username: "jwt_test_user".to_string(),
		password: "test_password".to_string(),
	};

	let auth_token = auth_service.login(credentials).await.unwrap();

	// Verify the token
	let token_data = auth_service.verify_token(&auth_token.token);
	assert!(token_data.is_ok(), "Token verification should succeed");

	let claims = token_data.unwrap().claims;
	assert_eq!(claims.username, "jwt_test_user");
	assert_eq!(claims.role, "MANAGER");

	// Test with invalid token
	let invalid_token_result = auth_service.verify_token("invalid.token.here");
	assert!(invalid_token_result.is_err(), "Invalid token should fail verification");
}
