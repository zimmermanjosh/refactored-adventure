use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::env;

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
    /// Run full CI pipeline
    Ci,
    /// Generate configuration file
    Config,
    /// Deploy to cloud platforms
    Deploy,
}

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() -> anyhow::Result<()> {
    // load env file if it exists
    dotenv().ok();
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => build()?,
        Commands::Clean => clean()?,
        Commands::Test => test()?,
        Commands::Run => run()?,
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

    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist)
            .context("Failed to clean dist folder")?;
        println!("Removed dist/");
    }

    let xtaskdist = Path::new("xtask/dist");
    if xtaskdist.exists() {
        fs::remove_dir_all(xtaskdist)
            .context("Failed to clean xtask/dist folder")?;
        println!("Removed xtask/dist/");
    }

    let assets = Path::new("assets");
    if assets.exists() {
        fs::remove_dir_all(assets)
            .context("Failed to clean assets folder")?;
        println!("Removed assets/");
    }

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
    unimplemented!("AWS deployment not ready yet");
    // TODO:: Write actual test
    /*let status = Command::new("cargo")
        .args(&["test", "-p", "flappy_game"])
        .status()
        .context("Failed to run cargo test")?;

    if !status.success() {
        anyhow::bail!("Tests failed with non-zero exit code");
    }

    println!("All tests passed!");
    Ok(())*/
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

fn ci() -> anyhow::Result<()> {
    println!("🚀 Running full CI pipeline...");
    
    println!("Step 1: Cleaning...");
    clean()?;
    
    println!("Step 2: Generating config...");
    config()?;

    // TODO:: this needs work
    println!("Step 3: Running tests...");
    test()?;
    
    println!("Step 4: Building...");
    build()?;

    // TODO:: this needs work 
    println!("Step 5: Deploying...");
    deploy()?;
    
    println!("✅ CI pipeline completed successfully!");
    Ok(())
}

fn deploy() -> anyhow::Result<()> {
    println!("🌩️ Deploying to cloud platforms...");

    // Example environment variable usage
    let aws_region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let deploy_env = env::var("DEPLOY_ENV").unwrap_or_else(|_| "dev".to_string());

    println!("Deploying to {} in region {}", deploy_env, aws_region);

    // TODO: Implement actual cloud deployment
    todo!("Implement cloud deployment with environment configuration");
    
    Ok(())
}