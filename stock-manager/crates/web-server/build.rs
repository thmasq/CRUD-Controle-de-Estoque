use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
	let files = [
		("https://unpkg.com/htmx.org@2.0.4", "htmx.min.js"),
		("https://cdn.tailwindcss.com", "tailwind.min.js"),
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

	for (url, filename) in files.iter() {
		let path: PathBuf = static_dir.join(filename);

		if force || !path.exists() {
			println!("cargo:warning=Downloading {filename}...");

			let content = client
				.get(*url)
				.send()
				.and_then(|res| res.error_for_status())
				.expect(&format!("Failed to download {}", filename))
				.bytes()
				.expect(&format!("Failed to read {} response body", filename));

			fs::write(&path, &content).expect(&format!("Failed to write {}", filename));
			println!("cargo:warning={} downloaded successfully", filename);
		}

		println!("cargo:rerun-if-changed={}", path.display());
	}

	println!("cargo:rerun-if-env-changed=FORCE_DOWNLOAD");
}
