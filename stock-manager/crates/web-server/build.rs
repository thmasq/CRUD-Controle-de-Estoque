// crates/web-server/build.rs
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use std::path::Path;
use std::{env, fs};

fn main() {
	let static_dir = Path::new("static");
	if !static_dir.exists() {
		fs::create_dir_all(static_dir).expect("Failed to create static directory");
	}

	let mut headers = HeaderMap::new();
	headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
	headers.insert(ACCEPT, HeaderValue::from_static("text/css,*/*;q=0.1"));
	headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

	let client = Client::builder()
		.redirect(reqwest::redirect::Policy::custom(|attempt| {
			if attempt.previous().len() > 10 {
				attempt.error("too many redirects")
			} else {
				attempt.follow()
			}
		}))
		.default_headers(headers)
		.build()
		.expect("Failed to create HTTP client");

	// Download htmx.min.js if it doesn't exist or if we're forcing redownload
	let htmx_path = static_dir.join("htmx.min.js");
	if !htmx_path.exists() || env::var("FORCE_DOWNLOAD").is_ok() {
		println!("cargo:warning=Downloading htmx.min.js...");
		let htmx_url = "https://unpkg.com/htmx.org/dist/htmx.min.js";

		let htmx_content = client
			.get(htmx_url)
			.send()
			.expect("Failed to download htmx.min.js")
			.bytes()
			.expect("Failed to read htmx.min.js response body")
			.to_vec();

		fs::write(&htmx_path, htmx_content).expect("Failed to write htmx.min.js to static directory");

		println!("cargo:warning=htmx.min.js downloaded successfully");
	}

	// Download tailwind.min.css if it doesn't exist or if we're forcing redownload
	let tailwind_path = static_dir.join("tailwind.min.css");
	if !tailwind_path.exists() || env::var("FORCE_DOWNLOAD").is_ok() {
		println!("cargo:warning=Downloading tailwind.min.css...");

		let tailwind_url = "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css";

		let tailwind_response = client
			.get(tailwind_url)
			.send()
			.expect("Failed to download tailwind.min.css");

		let tailwind_content = tailwind_response
			.bytes()
			.expect("Failed to read tailwind.min.css response body")
			.to_vec();

		fs::write(&tailwind_path, tailwind_content).expect("Failed to write tailwind.min.css to static directory");

		println!("cargo:warning=tailwind.min.css downloaded successfully");
	}

	// Tell Cargo to rerun this build script if the static files change
	println!("cargo:rerun-if-changed=static/htmx.min.js");
	println!("cargo:rerun-if-changed=static/tailwind.min.css");
	println!("cargo:rerun-if-env-changed=FORCE_DOWNLOAD");
}
