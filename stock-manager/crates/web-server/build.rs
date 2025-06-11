use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
	let files = [
		("https://unpkg.com/htmx.org@2.0.4", "htmx.min.js"),
		("https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4", "tailwind.min.js"),
	];

	let static_dir = Path::new("static");
	if !static_dir.exists() {
		fs::create_dir_all(static_dir).expect("Failed to create static directory");
	}

	let mut headers = HeaderMap::new();
	headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
	headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
	headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

	let client = Client::builder()
		.redirect(reqwest::redirect::Policy::limited(10))
		.default_headers(headers)
		.build()
		.expect("Failed to create HTTP client");

	let force = env::var("FORCE_DOWNLOAD").is_ok();

	for (url, filename) in &files {
		let path: PathBuf = static_dir.join(filename);

		if force || !path.exists() {
			println!("cargo:warning=Downloading {filename}...");

			let content = client
				.get(*url)
				.send()
				.and_then(reqwest::blocking::Response::error_for_status)
				.unwrap_or_else(|_| panic!("Failed to download {filename}"))
				.bytes()
				.unwrap_or_else(|_| panic!("Failed to read {filename} response body"));

			fs::write(&path, &content).unwrap_or_else(|_| panic!("Failed to write {filename}"));
			println!("cargo:warning={filename} downloaded successfully");
		}

		println!("cargo:rerun-if-changed={}", path.display());
	}

	println!("cargo:rerun-if-env-changed=FORCE_DOWNLOAD");
}
