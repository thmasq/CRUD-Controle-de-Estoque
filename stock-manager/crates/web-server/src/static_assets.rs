// stock-manager/crates/web-server/src/static_assets.rs
use actix_web::{HttpResponse, Result, web};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticAssets;

async fn get_static_file(path: web::Path<String>) -> Result<HttpResponse> {
	let path = path.into_inner();

	if let Some(content) = StaticAssets::get(&path) {
		let content_type = match path.split('.').last() {
			Some("js") => "application/javascript",
			Some("css") => "text/css",
			Some("woff") => "font/woff",
			Some("woff2") => "font/woff2",
			Some("ttf") => "font/ttf",
			Some("eot") => "application/vnd.ms-fontobject",
			Some("svg") => "image/svg+xml",
			Some("ico") => "image/x-icon",
			_ => "application/octet-stream",
		};

		Ok(HttpResponse::Ok()
			.content_type(content_type)
			.append_header(("Cache-Control", "max-age=31536000")) // Cache for 1 year
			.body(content.data.to_vec()))
	} else {
		Ok(HttpResponse::NotFound().body("File not found"))
	}
}

pub fn register(config: &mut web::ServiceConfig) {
	config.service(web::resource("/_static/{filename:.*}").route(web::get().to(get_static_file)));
}
