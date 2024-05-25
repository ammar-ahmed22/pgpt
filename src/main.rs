use anyhow::Context;
use pgpt::config;

fn main() -> anyhow::Result<()> {
    // let cli = pgpt::config::CLI::parse();
    // println!("{:?}", cli);
    match config::Config::parse_args() {
        config::ParsedArgs::Query { args } => {
            let config = config::Config::load_config()
                .with_context(|| format!("Failed to load config."))?;
            pgpt::run_query(args, config)
        },
        config::ParsedArgs::Config { config } => {
            config::Config::handle_config(&config)
        }
    }
    // let args = config::Config::parse_args();
    // // let args_arc = Arc::new(args);
    // if args.clear {
    //     config::Config::clear_config()?;
    //     return Ok(());
    // }
    // let config = config::Config::load_config();
    // pgpt::run(args, config)
    //     .with_context(|| format!("Failed to make query"))?;
    // Ok(())
}
