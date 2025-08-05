// ============================================================================
// External Crate Imports (std first, then external crates)
// ============================================================================

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::Context;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

// ============================================================================
// Type Definitions
// ============================================================================

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Dev automation for flappy dragon")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the flappy_game project
    Build,
    /// Remove dist, target, and assets folders
    Clean,
    /// Run all tests
    Test,
    /// Run the game
    Run,
    /// Watch for changes and auto-rebuild game
    Watch,
    /// Run full CI pipeline
    Ci,
    /// Generate configuration file
    Config,
    /// Deploy to cloud platforms
    Deploy,
}

// ============================================================================
// Constants
// ============================================================================

const DEFAULT_AWS_REGION: &str = "us-east-1";
const DEFAULT_DEPLOY_ENV: &str = "dev";

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() -> anyhow::Result<()> {
    // Load .env file if it exists (ignore errors if missing)
    dotenv().ok();
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => build()?,
        Commands::Clean => clean()?,
        Commands::Test => test()?,
        Commands::Run => run()?,
        Commands::Watch => watch()?,
        Commands::Ci => ci()?,
        Commands::Config => config()?,
        Commands::Deploy => deploy()?,
    }
    Ok(())
}

// ============================================================================
// Command Implementations
// ============================================================================

fn build() -> anyhow::Result<()> {
    println!("Running game build...");

    let status = Command::new("cargo")
        .args(&["build", "-p", "flappy_game"])
        .status()
        .context("Failed to run cargo build")?;

    if !status.success() {
        anyhow::bail!("Build process failed with non-zero exit code");
    }

    let dist_path = Path::new("dist");
    if !dist_path.exists() {
        fs::create_dir(dist_path)
            .context("Failed to create dist directory")?;
    }

    println!("Build complete. Output in /dist (manual copy or link if needed).");
    Ok(())
}

fn clean() -> anyhow::Result<()> {
    println!("Cleaning /dist, /target, /assets...");

    // Clean main dist folder
    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist)
            .context("Failed to clean dist folder")?;
        println!("Removed dist/");
    }

    // Clean xtask dist folder
    let xtask_dist = Path::new("xtask/dist");
    if xtask_dist.exists() {
        fs::remove_dir_all(xtask_dist)
            .context("Failed to clean xtask/dist folder")?;
        println!("Removed xtask/dist/");
    }

    // Clean assets folder
    let assets = Path::new("assets");
    if assets.exists() {
        fs::remove_dir_all(assets)
            .context("Failed to clean assets folder")?;
        println!("Removed assets/");
    }

    // Clean target folder
    let targets = Path::new("target");
    if targets.exists() {
        fs::remove_dir_all(targets)
            .context("Failed to clean targets folder")?;
        println!("Removed target/");
    }
    
    Ok(())
}

fn config() -> anyhow::Result<()> {
    let config_data = r#"
{
    "project": "flappy_game",
    "version": "0.1.0",
    "description": "Hands-On Rust game project"
}
"#;

    let dist_path = Path::new("dist");
    if !dist_path.exists() {
        fs::create_dir(dist_path).context("Failed to create dist folder")?;
    }

    let config_path = dist_path.join("config.json");
    let mut file = File::create(config_path).context("Failed to create config.json")?;
    file.write_all(config_data.as_bytes()).context("Failed to write to config.json")?;

    println!("Generated dist/config.json");
    Ok(())
}

fn test() -> anyhow::Result<()> {
    println!("Running game tests...");
    
    // TODO: Write actual test implementation
    // FIXME: Remove unimplemented when ready to implement tests
    unimplemented!("Test implementation not ready yet");
    
    // NOTE: Commented out until test implementation is ready
    /*
    let status = Command::new("cargo")
        .args(&["test", "-p", "flappy_game"])
        .status()
        .context("Failed to run cargo test")?;

    if !status.success() {
        anyhow::bail!("Tests failed with non-zero exit code");
    }

    println!("All tests passed!");
    Ok(())
    */
}

fn run() -> anyhow::Result<()> {
    println!("Running the game...");
    
    let status = Command::new("cargo")
        .args(&["run", "-p", "flappy_game"])
        .status()
        .context("Failed to run the game")?;

    if !status.success() {
        anyhow::bail!("Game execution failed");
    }

    Ok(())
}

fn watch() -> anyhow::Result<()> {
    println!("🔄 Starting file watcher for live game development...");
    println!("💡 Edit flappy_game/src/main.rs and save to see changes!");
    println!("🛑 Press Ctrl+C to stop watching");
    
    let status = Command::new("cargo")
        .args(&[
            "watch",
            "-c",                    // Clear screen before each rebuild
            "-w", "flappy_game/src", // Watch flappy_game source directory
            "-w", "flappy_game/Cargo.toml", // Watch for dependency changes
            "-x", "run -p flappy_game"       // Execute this command on changes
        ])
        .status()
        .context("Failed to start cargo watch - make sure it's installed with: cargo install cargo-watch")?;

    if !status.success() {
        anyhow::bail!("Cargo watch failed");
    }

    Ok(())
}

fn ci() -> anyhow::Result<()> {
    println!("🚀 Running full CI pipeline...");
    
    println!("Step 1: Cleaning...");
    clean()?;
    
    println!("Step 2: Generating config...");
    config()?;

    // TODO: Enable tests when implementation is ready
    // NOTE: Commented out to prevent CI pipeline from failing on unimplemented tests
    /*
    println!("Step 3: Running tests...");
    test()?;
    */
    
    println!("Step 3: Building...");
    build()?;

    // TODO: Enable deployment when implementation is ready
    // NOTE: Commented out to prevent CI pipeline from failing on todo!() macro
    /*
    println!("Step 4: Deploying...");
    deploy()?;
    */
    
    println!("✅ CI pipeline completed successfully!");
    Ok(())
}

fn deploy() -> anyhow::Result<()> {
    println!("🌩️ Deploying to cloud platforms...");

    // Load environment variables with fallback defaults
    let aws_region = env::var("AWS_REGION")
        .unwrap_or_else(|_| DEFAULT_AWS_REGION.to_string());
    let deploy_env = env::var("DEPLOY_ENV")
        .unwrap_or_else(|_| DEFAULT_DEPLOY_ENV.to_string());

    println!("Deploying to {} environment in region {}", deploy_env, aws_region);

    // TODO: Implement AWS S3 deployment
    // TODO: Add Google Cloud Storage support
    // TODO: Add authentication handling
    // TODO: Add deployment verification
    todo!("Implement cloud deployment with environment configuration");
}