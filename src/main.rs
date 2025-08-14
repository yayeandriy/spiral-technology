mod app;
mod pages;
mod projects;
mod areas;
mod catalog;
mod supabase;
use app::*;
use leptos::{logging, mount};

// pub const API_URL: &str = "https://api.spector.vision/api/v1/";

pub fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    // println!("cargo:rerun-if-env-changed=API_URL");
    // println!("cargo:rustc-env=API_URL={}", std::env::var("API_URL").unwrap_or_else(|_| "https://spector-next-api.vercel.app/api/v1/defects".to_string()));

    mount::mount_to_body(App);
}
