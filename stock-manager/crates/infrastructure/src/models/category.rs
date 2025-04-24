use chrono::{DateTime, Utc};
use diesel::prelude::*;
use stock_domain::entities::category::Category;
use uuid::Uuid;

use crate::schema::categories;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CategoryModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCategoryModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl From<CategoryModel> for Category {
    fn from(model: CategoryModel) -> Self {
        Category {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<Category> for NewCategoryModel {
    fn from(entity: Category) -> Self {
        NewCategoryModel {
            id: entity.id,
            name: entity.name,
            description: entity.description,
        }
    }
}
