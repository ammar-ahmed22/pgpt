pub mod model;
pub mod utils;

use crate::config::model::Model;
use crate::encryption::{encrypt, nonce};
use clap::Parser;
use colored::*;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(
  name = "pgpt",
  about = "Ask ChatGPT anything directly from the terminal with pretty markdown rendering!",
  version = env!("CARGO_PKG_VERSION"),
  author = "Ammar Ahmed <ammar.ahmed2203@gmail.com>"
)]
pub struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Make a query to ChatGPT
    Query {
        /// The query to ask ChatGPT
        #[arg(required = false)]
        query: Vec<String>,

        /// Display the total cost associated with prompt/response
        #[arg(long)]
        cost: bool,

        /// Use a specific model for the query (optional).
        #[arg(long, short, value_enum)]
        model: Option<Model>,

        /// [0] The number of previous prompt/response pairs to include in the query. Cannot exceed `cache-length` setting in configuration. (optional)
        #[arg(long, short)]
        context: Option<usize>,

        /// Display the context that is being passed with the query
        #[arg(long, short)]
        show_context: bool
    },
    /// Configure settings for using the CLI
    Config {
        #[command(subcommand)]
        config_commands: ConfigCommands,
    },
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set configuration settings
    Set {
        #[command(subcommand)]
        config_setters: ConfigSetters,
    },
    /// Show configuration settings
    Show {
        #[command(subcommand)]
        config_settings: ConfigSettings,
    },
    /// Clear configuration settings
    Clear {
        #[command(subcommand)]
        config_removers: ConfigRemovers,
    },
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigSetters {
    /// The model to use for ChatGPT [`gpt-3`, `gpt-4`, `gpt-4o`]
    Model {
        #[arg(value_enum)]
        value: Model,
    },
    /// The API key for OpenAI. Create one at https://platform.openai.com/api-keys
    APIKey { value: String },
    /// The maximum number of prompt/response pairs to save in cache.
    CacheLength { value: usize },
    /// The default number of prompt/response pairs to send as context with the query
    Context { value: usize },
}

impl ConfigSetters {
    pub fn set(&self) -> anyhow::Result<()> {
        let mut config = utils::load_config_file()?;

        match self {
            Self::APIKey { value } => match value.trim() {
                value if !value.is_empty() => {
                    utils::save_api_key(&value)?;
                    return Ok(());
                }
                e => return Err(anyhow::anyhow!("Received empty API key - {}", e)),
            },
            Self::CacheLength { value } => {
                println!(
                    "Setting {} to {}",
                    "cache-length".cyan(),
                    value.to_string().cyan()
                );
                config.cache_length = *value;
            }
            Self::Model { value } => {
                let str_model = value.to_string();
                println!("Setting {} to {}", "model".cyan(), str_model.cyan());
                config.model = str_model;
            }
            Self::Context { value } => {
                println!(
                    "Setting {} to {}",
                    "context".cyan(),
                    value.to_string().cyan()
                );
                config.context = *value;
            }
        };
        utils::save_config_file(&config)?;
        Ok(())
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigRemovers {
    /// Clear the saved API key
    APIKey,
    /// Clear the saved prompt/response cache
    Cache,
}

impl ConfigRemovers {
    pub fn clear(&self) -> anyhow::Result<()> {
        match self {
            Self::APIKey => utils::clear_api_key(),
            Self::Cache => utils::clear_cache(),
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigSettings {
    /// The model being used for ChatGPT
    Model,
    /// The API key for OpenAI (attempting to show the key will display an encrypted version!)
    APIKey,
    /// The maximum number of prompt/response pairs to save in cache.
    CacheLength,
    /// All of the previously saved prompt/response pairs
    Cache,
    /// The default number of prompt/response pairs to send with the query
    Context,
    /// All of the configuration values.
    All,
}

impl ConfigSettings {
    pub fn show(&self) -> anyhow::Result<()> {
        let config = utils::load_config_file().unwrap_or_else(|_| {
            utils::register_config_file().unwrap_or_else(|e| panic!("{:#?}", e))
        });
        let api_key = utils::load_api_key()
            .unwrap_or_else(|_| utils::register_api_key().unwrap_or_else(|e| panic!("{:#?}", e)));
        let encrypted = encrypt(api_key, utils::encryption_password(), nonce()?)?;
        let enc_str = String::from_utf8_lossy(&encrypted);
        match self {
            Self::Model => {
                println!("{}: {}", "Model".cyan(), config.model);
            }
            Self::APIKey => {
                println!("{}: {}", "API Key (encrypted)".cyan(), enc_str);
            }
            Self::Cache => {
                let cache = utils::load_cache()?;
                println!("{}:", "Cache".cyan());
                for (i, value) in cache.iter().enumerate() {
                    println!("{}", format!("Cached {}/{}", i + 1, cache.len()).cyan());
                    println!("{}: {}", "You said".yellow(), value.prompt);
                    println!("{}:\n{}", "GPT said".magenta(), value.response);
                    println!("")
                }
            }
            Self::CacheLength => {
                println!("{}: {}", "Cache Length".cyan(), config.cache_length);
            }
            Self::Context => {
                println!("{}: {}", "Context".cyan(), config.context);
            }
            Self::All => {
                // let cache = utils::load_cache()?;
                println!("{}: {}", "Model".cyan(), config.model);
                println!("{}: {}", "API Key (encrypted)".cyan(), enc_str);
                println!("{}: {}", "Cache Length".cyan(), config.cache_length);
                println!("{}: {}", "Context".cyan(), config.context);
                println!("To display cache, run `{}`", "pgpt config show cache".cyan());
            }
        };
        Ok(())
    }
}

pub struct QueryArgs {
    pub model: Option<Model>,
    pub query: String,
    pub cost: bool,
    pub context: Option<usize>,
    pub show_context: bool
}

pub enum ParsedArgs {
    Query { args: Arc<QueryArgs> },
    Config { config: ConfigCommands },
}

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub model: Model,
    pub cache_length: usize,
    pub context: usize,
}

impl Config {
    /// Attempts to load config from env vars then config file, otherwise prompts for user to input key
    pub fn load_config() -> anyhow::Result<Arc<Self>> {
        let api_key = utils::load_api_key()
            .unwrap_or_else(|_| utils::register_api_key().unwrap_or_else(|e| panic!("{:#?}", e)));
        let config_json = utils::load_config_file().unwrap_or_else(|_| {
            utils::register_config_file().unwrap_or_else(|e| panic!("{:#?}", e))
        });
        let model = Model::from_str(&config_json.model).map_err(|e| anyhow::anyhow!("{:#?}", e))?;

        // Creating cache file if it doesn't exist (not loading because only to be used when needed)
        utils::register_cache()?;

        let config = Self {
            api_key,
            model,
            cache_length: config_json.cache_length,
            context: config_json.context,
        };
        Ok(Arc::new(config))
    }

    /// Parses CLI arguments
    pub fn parse_args() -> ParsedArgs {
        let cli = CLI::parse();
        match cli.command {
            Commands::Query {
                query,
                cost,
                model,
                context,
                show_context
            } => {
                let query = query.join(" ");
                // let context = context.unwrap_or(0);
                let args = QueryArgs {
                    query,
                    model,
                    cost,
                    context,
                    show_context
                };
                ParsedArgs::Query {
                    args: Arc::new(args),
                }
            }
            Commands::Config { config_commands } => ParsedArgs::Config {
                config: config_commands,
            },
        }
    }

    /// Handles logic for config commands
    pub fn handle_config(config_commands: &ConfigCommands) -> anyhow::Result<()> {
        match config_commands {
            ConfigCommands::Show { config_settings } => config_settings.show(),
            ConfigCommands::Set { config_setters } => config_setters.set(),
            ConfigCommands::Clear { config_removers } => config_removers.clear(),
        }
        // Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConfigJSON {
    pub model: String,
    pub cache_length: usize,
    pub context: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CacheValue {
    pub prompt: String,
    pub response: String,
}
