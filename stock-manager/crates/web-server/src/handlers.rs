use actix_web::{HttpResponse, Responder, Result, http::header::ContentType, web};
use askama::Template;
// Remove askama_actix import
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use stock_application::ProductService;
use stock_domain::{DomainError, Product, ProductRepository, StockRepository};

// Template structs
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "products/index.html")]
struct ProductsTemplate {
    products: Vec<ProductViewModel>,
}

#[derive(Template)]
#[template(path = "stock/index.html")]
struct StockTemplate {
    stock_items: Vec<StockItemViewModel>,
    products: Vec<ProductViewModel>,
}

#[derive(Debug, Clone, Serialize)]
struct ProductViewModel {
    id: String,
    name: String,
    description: String,
    sku: String,
}

impl From<Product> for ProductViewModel {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.to_string(),
            name: product.name,
            description: product.description.unwrap_or_default(),
            sku: product.sku,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct StockItemViewModel {
    id: String,
    product_id: String,
    product_name: String,
    location: String,
    quantity: i32,
    unit_cost: String,
    last_restocked: String,
}

// Form DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductForm {
    name: String,
    sku: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStockItemForm {
    product_id: String,
    location: String,
    quantity: i32,
    unit_cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuantityForm {
    quantity: i32,
}

// Error handling helpers
fn map_domain_error(err: DomainError) -> HttpResponse {
    match err {
        DomainError::NotFound(msg) => HttpResponse::NotFound().body(msg),
        DomainError::InvalidOperation(msg) => HttpResponse::BadRequest().body(msg),
        DomainError::InfrastructureError(msg) => {
            eprintln!("Infrastructure error: {}", msg);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

// Helper function to render template
fn render_template<T: Template>(template: &T) -> Result<HttpResponse> {
    let body = template.render().map_err(|err| {
        eprintln!("Template rendering error: {}", err);
        actix_web::error::ErrorInternalServerError("Template rendering error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

// Route handlers
pub async fn index() -> impl Responder {
    let template = IndexTemplate;
    render_template(&template)
}

// Products
pub async fn products_index<P, S>(service: web::Data<ProductService<P, S>>) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    match service.get_all_products().await {
        Ok(products) => {
            let view_models = products.into_iter().map(ProductViewModel::from).collect();
            let template = ProductsTemplate {
                products: view_models,
            };
            render_template(&template)
        }
        Err(err) => Ok(map_domain_error(err)),
    }
}

pub async fn create_product<P, S>(
    form: web::Form<CreateProductForm>,
    service: web::Data<ProductService<P, S>>,
) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    match service
        .create_product(
            form.name.clone(),
            form.description.clone(),
            form.sku.clone(),
        )
        .await
    {
        Ok(product) => {
            let view_model = ProductViewModel::from(product);
            // Return HTML snippet for the new row
            let html = format!(
                r##"
                <tr id="product-{}">
                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-right">
                        <button 
                            class="text-blue-600 hover:text-blue-900 mr-3"
                            hx-get="/products/{}/edit"
                            hx-target="#modal-container"
                            hx-swap="innerHTML"
                        >
                            Edit
                        </button>
                        <button 
                            class="text-red-600 hover:text-red-900"
                            hx-delete="/products/{}"
                            hx-confirm="Are you sure you want to delete this product?"
                            hx-target="#product-{}"
                            hx-swap="outerHTML"
                        >
                            Delete
                        </button>
                    </td>
                </tr>
                "##,
                view_model.id,
                view_model.sku,
                view_model.name,
                view_model.description,
                view_model.id,
                view_model.id,
                view_model.id
            );
            Ok(HttpResponse::Ok().body(html))
        }
        Err(err) => Ok(map_domain_error(err)),
    }
}

pub async fn delete_product<P, S>(
    id: web::Path<String>,
    service: web::Data<ProductService<P, S>>,
) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid UUID format")),
    };

    match service.delete_product(id).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Ok(map_domain_error(err)),
    }
}

// Stock items
pub async fn stock_index<P, S>(service: web::Data<ProductService<P, S>>) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    // Get all products for the dropdown
    let products = match service.get_all_products().await {
        Ok(products) => products.into_iter().map(ProductViewModel::from).collect(),
        Err(err) => return Ok(map_domain_error(err)),
    };

    // We need to get all stock items from all products
    let mut all_stock_items = Vec::new();
    for product in &products {
        let binding: &ProductViewModel = product;
        let id_str = &binding.id;
        let product_id = match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => continue, // Skip invalid UUIDs
        };

        match service.get_product_stock(product_id).await {
            Ok(stock_items) => {
                for item in stock_items {
                    all_stock_items.push(StockItemViewModel {
                        id: item.id.to_string(),
                        product_id: item.product_id.to_string(),
                        product_name: product.name.clone(),
                        location: item.location,
                        quantity: item.quantity,
                        unit_cost: item.unit_cost.to_string(),
                        last_restocked: item.last_restocked.format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }
            }
            Err(_) => continue, // Skip errors
        }
    }

    let template = StockTemplate {
        stock_items: all_stock_items,
        products,
    };
    render_template(&template)
}

pub async fn create_stock_item<P, S>(
    form: web::Form<CreateStockItemForm>,
    service: web::Data<ProductService<P, S>>,
) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    let product_id = match Uuid::parse_str(&form.product_id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid product ID")),
    };

    let unit_cost = match BigDecimal::from_str(&form.unit_cost.to_string()) {
        Ok(cost) => cost,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid unit cost")),
    };

    match service
        .add_stock_item(product_id, form.quantity, form.location.clone(), unit_cost)
        .await
    {
        Ok(stock_item) => {
            // Get product name
            let product = match service.get_product(product_id).await {
                Ok(p) => p,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            // Return HTML snippet for the new row
            let html = format!(
                r##"
                <tr id="stock-{}">
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-right">
                        <button 
                            class="text-blue-600 hover:text-blue-900 mr-3"
                            hx-get="/stock/{}/update-quantity"
                            hx-target="#modal-container"
                            hx-swap="innerHTML"
                        >
                            Update Quantity
                        </button>
                    </td>
                </tr>
                "##,
                stock_item.id,
                product.name,
                stock_item.location,
                stock_item.quantity,
                stock_item.unit_cost,
                stock_item.last_restocked.format("%Y-%m-%d %H:%M:%S"),
                stock_item.id
            );
            Ok(HttpResponse::Ok().body(html))
        }
        Err(err) => Ok(map_domain_error(err)),
    }
}

pub async fn update_quantity_form<P, S>(
    id: web::Path<String>,
    service: web::Data<ProductService<P, S>>,
) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid UUID format")),
    };

    // Find stock item
    let stock_item = match service.stock_repo.find_by_id(id).await {
        Ok(item) => item,
        Err(err) => return Ok(map_domain_error(err)),
    };

    // Return the form with current quantity pre-filled
    let html = format!(
        r##"
        <div id="quantity-form-modal" class="fixed inset-0 bg-gray-600 bg-opacity-50 flex items-center justify-center">
            <div class="bg-white rounded-lg p-8 max-w-md w-full">
                <h2 class="text-xl font-bold mb-4">Update Quantity</h2>
                
                <form id="quantity-form" hx-put="/stock/{}/quantity" hx-target="#stock-{}" hx-swap="outerHTML">
                    <div class="mb-4">
                        <label for="new_quantity" class="block text-sm font-medium text-gray-700">New Quantity</label>
                        <input type="number" id="new_quantity" name="quantity" min="0" required 
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                            value="{}">
                    </div>
                    
                    <div class="flex justify-end space-x-3">
                        <button type="button" id="cancel-quantity-btn" 
                            onclick="document.getElementById('quantity-form-modal').remove()"
                            class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300">
                            Cancel
                        </button>
                        <button type="submit" 
                            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
                            Update
                        </button>
                    </div>
                </form>
            </div>
        </div>
        "##,
        stock_item.id, stock_item.id, stock_item.quantity
    );
    Ok(HttpResponse::Ok().body(html))
}

pub async fn update_quantity<P, S>(
    id: web::Path<String>,
    form: web::Form<UpdateQuantityForm>,
    service: web::Data<ProductService<P, S>>,
) -> Result<HttpResponse>
where
    P: ProductRepository,
    S: StockRepository,
{
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid UUID format")),
    };

    match service.update_stock_quantity(id, form.quantity).await {
        Ok(stock_item) => {
            // Get product name
            let product = match service.get_product(stock_item.product_id).await {
                Ok(p) => p,
                Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
            };

            // Return HTML snippet for the updated row
            let html = format!(
                r##"
                <tr id="stock-{}">
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">{}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-right">
                        <button 
                            class="text-blue-600 hover:text-blue-900 mr-3"
                            hx-get="/stock/{}/update-quantity"
                            hx-target="#modal-container"
                            hx-swap="innerHTML"
                        >
                            Update Quantity
                        </button>
                    </td>
                </tr>
                "##,
                stock_item.id,
                product.name,
                stock_item.location,
                stock_item.quantity,
                stock_item.unit_cost,
                stock_item.last_restocked.format("%Y-%m-%d %H:%M:%S"),
                stock_item.id
            );
            Ok(HttpResponse::Ok().body(html))
        }
        Err(err) => Ok(map_domain_error(err)),
    }
}
