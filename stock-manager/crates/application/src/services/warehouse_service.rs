use std::sync::Arc;

use chrono::Utc;
use stock_domain::entities::warehouse::Warehouse;
use stock_domain::repositories::warehouse_repository::WarehouseRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WarehouseCreateDto {
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct WarehouseUpdateDto {
	pub id: Uuid,
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

pub struct WarehouseService {
	repository: Arc<dyn WarehouseRepository>,
}

impl WarehouseService {
	pub fn new(repository: Arc<dyn WarehouseRepository>) -> Self {
		Self { repository }
	}

	pub async fn get_warehouse(&self, id: Uuid) -> anyhow::Result<Option<Warehouse>> {
		self.repository.find_by_id(id).await
	}

	pub async fn get_all_warehouses(&self) -> anyhow::Result<Vec<Warehouse>> {
		self.repository.find_all().await
	}

	pub async fn create_warehouse(&self, dto: WarehouseCreateDto) -> anyhow::Result<Warehouse> {
		let now = Utc::now();
		let warehouse = Warehouse {
			id: Uuid::new_v4(),
			name: dto.name,
			location: dto.location,
			contact_info: dto.contact_info,
			is_active: dto.is_active,
			created_at: now,
			updated_at: now,
		};

		self.repository.create(warehouse).await
	}

	pub async fn update_warehouse(&self, dto: WarehouseUpdateDto) -> anyhow::Result<Warehouse> {
		let existing = self
			.repository
			.find_by_id(dto.id)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Warehouse not found"))?;

		let warehouse = Warehouse {
			id: existing.id,
			name: dto.name,
			location: dto.location,
			contact_info: dto.contact_info,
			is_active: dto.is_active,
			created_at: existing.created_at,
			updated_at: Utc::now(),
		};

		self.repository.update(warehouse).await
	}

	pub async fn delete_warehouse(&self, id: Uuid) -> anyhow::Result<bool> {
		self.repository.delete(id).await
	}
}
