pub mod config;
pub mod encryption;


pub fn run(args: &config::CLIArgs, config: &config::Config) -> anyhow::Result<()> {
  
  println!("Using model: `{}`", args.model);
  println!("Query: `{}`", args.query);
  println!("api_key: {}", config.api_key);

  Ok(())
}
