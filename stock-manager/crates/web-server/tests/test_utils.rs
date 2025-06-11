use actix_web::cookie::Cookie;
use actix_web::test;
use serde_json::Value;
use uuid::Uuid;

pub struct TestClient {
	pub auth_token: Option<String>,
	pub username: Option<String>,
}

#[allow(dead_code)]
impl TestClient {
	pub fn new() -> Self {
		Self {
			auth_token: None,
			username: None,
		}
	}

	pub fn with_auth_token(mut self, token: String, username: String) -> Self {
		self.auth_token = Some(token);
		self.username = Some(username);
		self
	}

	pub fn add_auth_headers(&self, mut req: test::TestRequest) -> test::TestRequest {
		if let Some(token) = &self.auth_token {
			req = req.cookie(Cookie::new("auth_token", token.clone()));
		}
		if let Some(username) = &self.username {
			req = req.cookie(Cookie::new("username", username.clone()));
		}
		req
	}

	pub fn extract_auth_from_response(
		&mut self,
		resp: &actix_web::dev::ServiceResponse,
	) -> Result<(), Box<dyn std::error::Error>> {
		for (name, value) in resp.headers() {
			if name.as_str().to_lowercase() == "set-cookie" {
				let cookie_str = value.to_str()?;

				if let Some(token_start) = cookie_str.find("auth_token=") {
					let token_start = token_start + "auth_token=".len();
					let token_end = cookie_str[token_start..]
						.find(';')
						.unwrap_or(cookie_str.len() - token_start);
					let token = &cookie_str[token_start..token_start + token_end];
					if !token.is_empty() {
						self.auth_token = Some(token.to_string());
					}
				}

				if let Some(username_start) = cookie_str.find("username=") {
					let username_start = username_start + "username=".len();
					let username_end = cookie_str[username_start..]
						.find(';')
						.unwrap_or(cookie_str.len() - username_start);
					let username = &cookie_str[username_start..username_start + username_end];
					if !username.is_empty() {
						self.username = Some(username.to_string());
					}
				}
			}
		}
		Ok(())
	}
}

#[allow(dead_code)]
pub struct TestData {
	pub categories: Vec<TestCategory>,
	pub warehouses: Vec<TestWarehouse>,
	pub products: Vec<TestProduct>,
}

#[allow(dead_code)]
pub struct TestCategory {
	pub id: Option<Uuid>,
	pub name: String,
	pub description: Option<String>,
}

#[allow(dead_code)]
pub struct TestWarehouse {
	pub id: Option<Uuid>,
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

#[allow(dead_code)]
pub struct TestProduct {
	pub id: Option<Uuid>,
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
}

#[allow(dead_code)]
impl TestData {
	pub fn sample() -> Self {
		Self {
			categories: vec![
				TestCategory {
					id: None,
					name: "Electronics".to_string(),
					description: Some("Electronic devices and components".to_string()),
				},
				TestCategory {
					id: None,
					name: "Books".to_string(),
					description: Some("Books and publications".to_string()),
				},
			],
			warehouses: vec![
				TestWarehouse {
					id: None,
					name: "Main Warehouse".to_string(),
					location: "123 Main Street, City".to_string(),
					contact_info: Some("main@warehouse.com".to_string()),
					is_active: true,
				},
				TestWarehouse {
					id: None,
					name: "Secondary Warehouse".to_string(),
					location: "456 Oak Avenue, Town".to_string(),
					contact_info: None,
					is_active: true,
				},
			],
			products: vec![
				TestProduct {
					id: None,
					name: "Laptop".to_string(),
					description: Some("High-performance laptop".to_string()),
					sku: "LAPTOP-001".to_string(),
					category_id: None, // Will be set after category creation
					is_active: true,
				},
				TestProduct {
					id: None,
					name: "Programming Book".to_string(),
					description: Some("Learn Rust programming".to_string()),
					sku: "BOOK-RUST-001".to_string(),
					category_id: None, // Will be set after category creation
					is_active: true,
				},
			],
		}
	}
}

/// Properly encode form data for Actix Web tests
#[allow(dead_code)]
pub fn form_encode(data: &serde_json::Value) -> String {
	let obj = data.as_object().unwrap();
	obj.iter()
		.filter_map(|(k, v)| {
			// Skip null values
			if v.is_null() {
				return None;
			}

			let value = match v {
				Value::String(s) => s.clone(),
				Value::Bool(b) => b.to_string(),
				Value::Number(n) => n.to_string(),
				_ => {
					// Convert other types to string and remove quotes
					let s = v.to_string();
					if s.starts_with('"') && s.ends_with('"') {
						s[1..s.len() - 1].to_string()
					} else {
						s
					}
				},
			};
			Some(format!("{}={}", urlencoding::encode(k), urlencoding::encode(&value)))
		})
		.collect::<Vec<_>>()
		.join("&")
}

/// Convert JSON object to form-safe data by removing nulls and converting types
#[allow(dead_code)]
pub fn to_form_data(mut data: serde_json::Value) -> serde_json::Value {
	if let Value::Object(ref mut map) = data {
		// Remove null values and convert empty strings for optional fields
		map.retain(|_, v| !v.is_null());

		// Convert specific boolean values to strings for form handling
		for (key, value) in map.iter_mut() {
			match value {
				Value::Bool(b) => {
					*value = Value::String(b.to_string());
				},
				Value::String(s) if s.is_empty() && key.contains("description") => {
					// Keep empty descriptions as empty strings
				},
				_ => {},
			}
		}
	}
	data
}

#[allow(dead_code)]
pub async fn wait_for_response_with_timeout(
	app: &impl actix_web::dev::Service<
		actix_web::dev::ServiceRequest,
		Response = actix_web::dev::ServiceResponse,
		Error = actix_web::Error,
	>,
	req: actix_web::test::TestRequest,
	timeout_ms: u64,
) -> actix_web::dev::ServiceResponse {
	let req = req.to_srv_request();
	tokio::time::timeout(
		std::time::Duration::from_millis(timeout_ms),
		test::call_service(app, req),
	)
	.await
	.expect("Request timed out")
}
