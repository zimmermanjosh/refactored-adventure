use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("build") => build(),
        Some("clean") => clean(),
        Some("config") => generate_config(),
        Some("help") | _ => help(),
    }
}

fn build() {
    println!("Running game build...");

    let status = Command::new("cargo")
        //.args(&["build", "-p", "flappy_game", "--release"])
        .args(&["build", "-p", "flappy_game", "--debug"])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        eprintln!("Build failed.");
        exit(1);
    }

    let dist_path = Path::new("dist");
    if !dist_path.exists() {
        fs::create_dir(dist_path).expect("Failed to create dist folder");
    }

    println!("Build complete. Output in /dist (manual copy or link if needed).");
}

fn clean() {
    println!("Cleaning /dist, /target, /assets...");

    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist).expect("Failed to clean dist folder");
        println!("Removed dist/");
    }

    let assets = Path::new("assets");
    if assets.exists() {
        fs::remove_dir_all(assets).expect("Failed to clean assets folder");
        println!("Removed assets/");
    }
    
    let targets = Path::new("target");
    if targets.exists() {
        fs::remove_dir_all(targets).expect("Failed to clean targets folder");
        println!("Removed target/");
    }
}

fn generate_config() {
    let config_data = r#"
{
    "project": "flappy_game",
    "version": "0.1.0",
    "description": "Hands-On Rust game project"
}
"#;

    let dist_path = Path::new("dist");
    if !dist_path.exists() {
        fs::create_dir(dist_path).expect("Failed to create dist folder");
    }

    let config_path = dist_path.join("config.json");
    let mut file = File::create(config_path).expect("Failed to create config.json");
    file.write_all(config_data.as_bytes()).expect("Failed to write to config.json");

    println!("Generated dist/config.json");
}

fn help() {
    println!("Available xtask commands:");
    println!("  build     - Build the flappy_game project (release)");
    println!("  clean     - Remove /dist, /target, /assets folders");
    println!("  config    - Generate a basic dist/config.json file");
    println!("  help      - Show this help message");
}