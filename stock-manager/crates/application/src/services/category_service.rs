use std::sync::Arc;

use chrono::Utc;
use stock_domain::entities::category::Category;
use stock_domain::repositories::category_repository::CategoryRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CategoryCreateDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CategoryUpdateDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

pub struct CategoryService {
    repository: Arc<dyn CategoryRepository>,
}

impl CategoryService {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_category(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_all_categories(&self) -> anyhow::Result<Vec<Category>> {
        self.repository.find_all().await
    }

    pub async fn create_category(&self, dto: CategoryCreateDto) -> anyhow::Result<Category> {
        let now = Utc::now();
        let category = Category {
            id: Uuid::new_v4(),
            name: dto.name,
            description: dto.description,
            created_at: now,
            updated_at: now,
        };

        self.repository.create(category).await
    }

    pub async fn update_category(&self, dto: CategoryUpdateDto) -> anyhow::Result<Category> {
        let existing = self
            .repository
            .find_by_id(dto.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Category not found"))?;

        let category = Category {
            id: existing.id,
            name: dto.name,
            description: dto.description,
            created_at: existing.created_at,
            updated_at: Utc::now(),
        };

        self.repository.update(category).await
    }

    pub async fn delete_category(&self, id: Uuid) -> anyhow::Result<bool> {
        self.repository.delete(id).await
    }
}
