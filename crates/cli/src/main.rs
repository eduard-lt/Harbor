use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "harbor")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate {
        #[arg(default_value = "harbor.config.yaml")]
        path: String,
    },
    Init {
        #[arg(default_value = "harbor.config.yaml")]
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Validate { path } => {
            let cfg = harbor_core::config::load_config(&path)?;
            harbor_core::config::validate_config(&cfg)?;
            println!("valid");
            Ok(())
        }
        Commands::Init { path } => init_config(&path),
    }
}

fn init_config(path: &str) -> Result<()> {
    let sample = r#"services:
  - name: web
    command: "node server.js"
    cwd: "."
    depends_on: []
    health_check:
      kind: http
      url: "http://localhost:3000/health"
      timeout_ms: 5000
      retries: 10
"#;
    std::fs::write(path, sample)?;
    println!("created {}", path);
    Ok(())
}
