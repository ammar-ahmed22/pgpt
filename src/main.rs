use anyhow::Context;
use pgpt::config;


fn main() -> anyhow::Result<()> {
    let args = config::Config::parse_args();
    if args.clear {
        config::Config::clear_config()?;
        return Ok(());
    }
    let config = config::Config::load_config();
    pgpt::run(&args, &config)
        .with_context(|| format!("Failed to make query"))?;
    Ok(())
}
