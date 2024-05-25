use anyhow::Context;
use pgpt::config;

fn main() -> anyhow::Result<()> {
    println!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    match config::Config::parse_args() {
        config::ParsedArgs::Query { args } => {
            let config =
                config::Config::load_config().with_context(|| format!("Failed to load config."))?;
            pgpt::run_query(args, config)
        }
        config::ParsedArgs::Config { config } => config::Config::handle_config(&config),
    }
}
