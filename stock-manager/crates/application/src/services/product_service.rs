use std::sync::Arc;

use chrono::Utc;
use stock_domain::entities::product::Product;
use stock_domain::repositories::product_repository::ProductRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProductCreateDto {
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct ProductUpdateDto {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
}

pub struct ProductService {
	repository: Arc<dyn ProductRepository>,
}

impl ProductService {
	pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
		Self { repository }
	}

	pub async fn get_product(&self, id: Uuid) -> anyhow::Result<Option<Product>> {
		self.repository.find_by_id(id).await
	}

	pub async fn get_product_by_sku(&self, sku: &str) -> anyhow::Result<Option<Product>> {
		self.repository.find_by_sku(sku).await
	}

	pub async fn get_all_products(&self) -> anyhow::Result<Vec<Product>> {
		self.repository.find_all().await
	}

	pub async fn get_products_by_category(&self, category_id: Uuid) -> anyhow::Result<Vec<Product>> {
		self.repository.find_by_category(category_id).await
	}

	pub async fn create_product(&self, dto: ProductCreateDto) -> anyhow::Result<Product> {
		// Check if product with the same SKU already exists
		if (self.repository.find_by_sku(&dto.sku).await?).is_some() {
			return Err(anyhow::anyhow!("Product with this SKU already exists"));
		}

		let now = Utc::now();
		let product = Product {
			id: Uuid::new_v4(),
			name: dto.name,
			description: dto.description,
			sku: dto.sku,
			category_id: dto.category_id,
			is_active: dto.is_active,
			created_at: now,
			updated_at: now,
		};

		self.repository.create(product).await
	}

	pub async fn update_product(&self, dto: ProductUpdateDto) -> anyhow::Result<Product> {
		let existing = self
			.repository
			.find_by_id(dto.id)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Product not found"))?;

		// If SKU changed, check if the new SKU is unique
		if existing.sku != dto.sku && (self.repository.find_by_sku(&dto.sku).await?).is_some() {
			return Err(anyhow::anyhow!("Product with this SKU already exists"));
		}

		let product = Product {
			id: existing.id,
			name: dto.name,
			description: dto.description,
			sku: dto.sku,
			category_id: dto.category_id,
			is_active: dto.is_active,
			created_at: existing.created_at,
			updated_at: Utc::now(),
		};

		self.repository.update(product).await
	}

	pub async fn delete_product(&self, id: Uuid) -> anyhow::Result<bool> {
		self.repository.delete(id).await
	}
}
