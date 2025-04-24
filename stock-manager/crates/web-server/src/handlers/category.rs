use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use uuid::Uuid;

use stock_application::services::category_service::{CategoryCreateDto, CategoryUpdateDto};

use crate::AppState;
use crate::dtos::category::{
    CategoryCreateRequest, CategoryDto, CategoryFormTemplate, CategoryListTemplate,
    CategoryUpdateRequest,
};

pub async fn list_categories(state: web::Data<AppState>) -> Result<HttpResponse> {
    let service = state.category_service.clone();
    let categories = service.get_all_categories().await.unwrap_or_default();

    // Count products in each category
    let product_service = state.product_service.clone();
    let products = product_service.get_all_products().await.unwrap_or_default();

    let mut category_dtos = Vec::new();
    for category in categories {
        let product_count = products
            .iter()
            .filter(|p| p.category_id.is_some() && p.category_id.unwrap() == category.id)
            .count() as i64;

        category_dtos.push(CategoryDto {
            id: category.id,
            name: category.name,
            description: category.description,
            product_count,
        });
    }

    let template = CategoryListTemplate {
        categories: category_dtos,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.dyn_render().unwrap()))
}

pub async fn new_category_form() -> Result<HttpResponse> {
    let template = CategoryFormTemplate {
        form_title: "Add Category".to_string(),
        form_action: "/categories".to_string(),
        form_method: "post".to_string(),
        category: CategoryDto {
            id: Uuid::nil(),
            name: "".to_string(),
            description: None,
            product_count: 0,
        },
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.dyn_render().unwrap()))
}

pub async fn edit_category_form(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let category_id = path.into_inner();
    let service = state.category_service.clone();

    if let Some(category) = service.get_category(category_id).await.unwrap_or(None) {
        let template = CategoryFormTemplate {
            form_title: "Edit Category".to_string(),
            form_action: format!("/categories/{}", category_id),
            form_method: "put".to_string(),
            category: CategoryDto {
                id: category.id,
                name: category.name,
                description: category.description,
                product_count: 0, // Not needed for form
            },
        };

        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(template.dyn_render().unwrap()))
    } else {
        Ok(HttpResponse::NotFound().body("Category not found"))
    }
}

pub async fn create_category(
    state: web::Data<AppState>,
    form: web::Form<CategoryCreateRequest>,
) -> Result<HttpResponse> {
    let service = state.category_service.clone();

    let dto = CategoryCreateDto {
        name: form.name.clone(),
        description: form.description.clone(),
    };

    match service.create_category(dto).await {
        Ok(category) => {
            // Return a row HTML for the new category to be added to the table
            let category_dto = CategoryDto {
                id: category.id,
                name: category.name,
                description: category.description,
                product_count: 0,
            };

            let html = format!(
                r##"<tr id="category-row-{id}">
                    <td class="px-6 py-4 whitespace-nowrap">{name}</td>
                    <td class="px-6 py-4">{desc}</td>
                    <td class="px-6 py-4 whitespace-nowrap">0</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">
                        <button 
                            class="text-indigo-600 hover:text-indigo-900 mr-3"
                            hx-get="/categories/{id}/edit"
                            hx-target="#modal-container"
                            hx-trigger="click">
                            Edit
                        </button>
                        <button 
                            class="text-red-600 hover:text-red-900"
                            hx-delete="/categories/{id}"
                            hx-confirm="Are you sure you want to delete this category?"
                            hx-target="#category-row-{id}"
                            hx-swap="outerHTML"
                            hx-trigger="click">
                            Delete
                        </button>
                    </td>
                </tr>"##,
                id = category_dto.id,
                name = category_dto.name,
                desc = category_dto.description.unwrap_or_default()
            );

            Ok(HttpResponse::Ok()
                .append_header(("HX-Trigger", "itemCreated"))
                .content_type("text/html")
                .body(html))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to create category: {}", e)))
        }
    }
}

pub async fn update_category(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    form: web::Form<CategoryUpdateRequest>,
) -> Result<HttpResponse> {
    let category_id = path.into_inner();
    let service = state.category_service.clone();

    let dto = CategoryUpdateDto {
        id: category_id,
        name: form.name.clone(),
        description: form.description.clone(),
    };

    match service.update_category(dto).await {
        Ok(_category) => Ok(HttpResponse::Ok()
            .append_header(("HX-Trigger", "itemUpdated"))
            .finish()),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to update category: {}", e)))
        }
    }
}

pub async fn delete_category(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let category_id = path.into_inner();
    let service = state.category_service.clone();

    match service.delete_category(category_id).await {
        Ok(true) => Ok(HttpResponse::Ok()
            .append_header(("HX-Trigger", "itemDeleted"))
            .finish()),
        Ok(false) => Ok(HttpResponse::NotFound().body("Category not found")),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to delete category: {}", e)))
        }
    }
}
