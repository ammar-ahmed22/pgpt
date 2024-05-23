use anyhow::Context;
use pgpt::config;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    // TODO refactor to make parse_args return Arc
    let args = config::Config::parse_args();
    let args_arc = Arc::new(args);
    if args_arc.clear {
        config::Config::clear_config()?;
        return Ok(());
    }
    // TODO refactor to make load_config return Arc
    let config = config::Config::load_config();
    let config_arc = Arc::new(config);
    pgpt::run(args_arc, config_arc)
        .with_context(|| format!("Failed to make query"))?;
    Ok(())
}
