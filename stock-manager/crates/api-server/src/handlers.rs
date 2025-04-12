use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use stock_application::ProductService;
use stock_domain::DomainError;
use stock_infrastructure::{PostgresProductRepository, PostgresStockRepository};

use crate::dtos::{
    ApiError, CreateProductRequest, CreateStockItemRequest, ProductResponse, StockItemResponse,
    UpdateProductRequest, UpdateStockQuantityRequest,
};

// Type alias for our concrete ProductService implementation
type StockProductService = ProductService<PostgresProductRepository, PostgresStockRepository>;

// Convert domain error to HTTP response
fn map_domain_error(err: DomainError) -> HttpResponse {
    match err {
        DomainError::NotFound(msg) => HttpResponse::NotFound().json(ApiError { message: msg }),
        DomainError::InvalidOperation(msg) => {
            HttpResponse::BadRequest().json(ApiError { message: msg })
        }
        DomainError::InfrastructureError(msg) => {
            eprintln!("Infrastructure error: {}", msg);
            HttpResponse::InternalServerError().json(ApiError {
                message: "Internal server error".to_string(),
            })
        }
    }
}

// Convert domain model to response DTO
fn product_to_response(product: stock_domain::Product) -> ProductResponse {
    ProductResponse {
        id: product.id.to_string(),
        name: product.name,
        description: product.description,
        sku: product.sku,
        created_at: product.created_at.to_rfc3339(),
        updated_at: product.updated_at.to_rfc3339(),
    }
}

// Convert domain model to response DTO
fn stock_item_to_response(stock_item: stock_domain::StockItem) -> StockItemResponse {
    StockItemResponse {
        id: stock_item.id.to_string(),
        product_id: stock_item.product_id.to_string(),
        quantity: stock_item.quantity,
        location: stock_item.location,
        unit_cost: stock_item.unit_cost.to_string(),
        last_restocked: stock_item.last_restocked.to_rfc3339(),
    }
}

// Product endpoints
#[get("/products")]
pub async fn get_products(service: web::Data<StockProductService>) -> impl Responder {
    match service.get_all_products().await {
        Ok(products) => {
            let response = products
                .into_iter()
                .map(product_to_response)
                .collect::<Vec<_>>();
            HttpResponse::Ok().json(response)
        }
        Err(err) => map_domain_error(err),
    }
}

#[get("/products/{id}")]
pub async fn get_product(
    id: web::Path<String>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiError {
                message: "Invalid UUID format".to_string(),
            });
        }
    };

    match service.get_product(id).await {
        Ok(product) => HttpResponse::Ok().json(product_to_response(product)),
        Err(err) => map_domain_error(err),
    }
}

#[post("/products")]
pub async fn create_product(
    request: web::Json<CreateProductRequest>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    match service
        .create_product(
            request.name.clone(),
            request.description.clone(),
            request.sku.clone(),
        )
        .await
    {
        Ok(product) => HttpResponse::Created().json(product_to_response(product)),
        Err(err) => map_domain_error(err),
    }
}

#[put("/products/{id}")]
pub async fn update_product(
    id: web::Path<String>,
    request: web::Json<UpdateProductRequest>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiError {
                message: "Invalid UUID format".to_string(),
            });
        }
    };

    match service
        .update_product(
            id,
            request.name.clone(),
            request.description.clone(),
            request.sku.clone(),
        )
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product_to_response(product)),
        Err(err) => map_domain_error(err),
    }
}

#[delete("/products/{id}")]
pub async fn delete_product(
    id: web::Path<String>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiError {
                message: "Invalid UUID format".to_string(),
            });
        }
    };

    match service.delete_product(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => map_domain_error(err),
    }
}

// Stock item endpoints
#[get("/products/{id}/stock")]
pub async fn get_product_stock(
    id: web::Path<String>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiError {
                message: "Invalid UUID format".to_string(),
            });
        }
    };

    match service.get_product_stock(id).await {
        Ok(stock_items) => {
            let response = stock_items
                .into_iter()
                .map(stock_item_to_response)
                .collect::<Vec<_>>();
            HttpResponse::Ok().json(response)
        }
        Err(err) => map_domain_error(err),
    }
}

#[post("/stock")]
pub async fn create_stock_item(
    request: web::Json<CreateStockItemRequest>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    // Convert f64 to BigDecimal
    let unit_cost = match BigDecimal::from_str(&request.unit_cost.to_string()) {
        Ok(cost) => cost,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiError {
                message: "Invalid unit cost format".to_string(),
            });
        }
    };

    match service
        .add_stock_item(
            request.product_id,
            request.quantity,
            request.location.clone(),
            unit_cost,
        )
        .await
    {
        Ok(stock_item) => HttpResponse::Created().json(stock_item_to_response(stock_item)),
        Err(err) => map_domain_error(err),
    }
}

#[put("/stock/{id}/quantity")]
pub async fn update_stock_quantity(
    id: web::Path<String>,
    request: web::Json<UpdateStockQuantityRequest>,
    service: web::Data<StockProductService>,
) -> impl Responder {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiError {
                message: "Invalid UUID format".to_string(),
            });
        }
    };

    match service.update_stock_quantity(id, request.quantity).await {
        Ok(stock_item) => HttpResponse::Ok().json(stock_item_to_response(stock_item)),
        Err(err) => map_domain_error(err),
    }
}
