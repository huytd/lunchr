mod app;
mod config;

use app::LunchrApp;
use config::Config;

fn main() {
    match Config::load() {
        Ok(config) => LunchrApp::new(config).run(),
        Err(e) => println!("Error loading config: {}", e),
    }
}
