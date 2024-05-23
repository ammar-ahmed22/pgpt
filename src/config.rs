use std::io::{Read, Write};
use clap::Parser;
use crate::encryption::{ decrypt, encrypt, nonce };
use anyhow::Context;
use colored::*;
use std::str::FromStr;

#[derive(Debug, Parser, Clone, serde::Serialize)]
pub enum Model {
  GPT3,
  GPT4,
  GPT4o
}

impl FromStr for Model {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "gpt-3" => Ok(Model::GPT3),
        "gpt-4" => Ok(Model::GPT4),
        "gpt-4o" => Ok(Model::GPT4o),
        _ => Err(format!("'{}' is not a valid model", s))
      }
  }
}

impl Model {
  pub fn api_model(&self) -> String {
    match self {
      Self::GPT3 => String::from("gpt-3.5-turbo"),
      Self::GPT4 => String::from("gpt-4-turbo"),
      Self::GPT4o => String::from("gpt-4o")
    }
  }

  /// Returns the price ($) per token for the model input
  pub fn prompt_cost(&self) -> f64 {
    match self {
      Self::GPT3 => 0.5 / 1e6,
      Self::GPT4 => 10.0 / 1e6,
      Self::GPT4o => 5.0 / 1e6
    }
  }

  /// Returns the price ($) per token for the model output
  pub fn completion_cost(&self) -> f64 {
    match self {
      Self::GPT3 => 1.5 / 1e6,
      Self::GPT4 => 30.0 / 1e6,
      Self::GPT4o => 15.0 / 1e6
    }
  }
}

#[derive(Debug, Parser)]
struct Args {
  /// The query to ask ChatGPT
  #[arg(required = false)]
  query: Vec<String>,

  /// ['gpt-3'] The model to use (optional) 
  /// [options = 'gpt-3', 'gpt-4', 'gpt-4o']
  #[arg(long, short, value_enum)] 
  model: Option<Model>,

  /// Remove local config including OpenAI API key
  #[arg(long)]
  clear: bool
}

#[derive(serde::Serialize, Debug)]
pub struct CLIArgs {
  pub model: Model,
  pub clear: bool,
  pub query: String,
}

#[derive(Debug)]
pub struct Config {
  pub api_key: String,
}

impl Config {

  /// Attempts to load config from env vars then config file, otherwise prompts for user to input key
  pub fn load_config() -> Self {
    let api_key = load_api_key().unwrap_or_else(|_| register_api_key().unwrap_or_else(|e| panic!("{:#?}", e)));
    Self { api_key }
  }

  /// Parses CLI arguments
  pub fn parse_args() -> CLIArgs {
    let args = Args::parse();
    let query = args.query.join(" ");
    let model = match args.model {
      Some(m) => m,
      None => Model::GPT3
    };

    CLIArgs {
      model,
      query,
      clear: args.clear,
    }
  }

  /// Clears the local config file.
  pub fn clear_config() -> anyhow::Result<()> {
    let config_path = config_file_path();
    std::fs::remove_file(&config_path)?;
    println!("Removed config file at {}", format!("{:?}", config_path).cyan());
    Ok(())
  }
}


/// Attempts to load API key from env variable or config file
fn load_api_key() -> anyhow::Result<String> {
  if let Ok(key) = read_api_key_from_env() {
    return Ok(key);
  }

  let config_path = config_file_path();
  let mut buffer = Vec::new();
  let mut file = std::fs::File::open(&config_path)?;
  file.read_to_end(&mut buffer)?;
  let api_key_buf = decrypt(buffer, encryption_password())?;
  let api_key = String::from_utf8(api_key_buf)?;

  Ok(api_key)
}

fn register_api_key() -> anyhow::Result<String> {
  println!();
  println!("OpenAI API key not found!");
  println!();
  println!("You need to enter an {} which will be encrypted and saved locally!", "OpenAI API key".cyan());
  println!("You can create an API key at https://platform.openai.com/api-keys");
  println!();
  println!("If you don't want to save it, you can pass your API key to the environment variable `{}`!", "OPENAI_API_KEY".cyan());
  println!();
  println!();
  println!("{}", "Enter your API key:".bright_cyan());
  let mut api_key = String::new();
  std::io::stdin()
    .read_line(&mut api_key)
    .context("Failed to read input")?;

  match api_key.trim() {
    api_key if !api_key.is_empty() => {
      save_api_key(&api_key)?;
      Ok(api_key.to_string())
    }
    e => Err(anyhow::anyhow!("Received empty API key - {}", e))
  }
  // Ok(String::new())
}

fn save_api_key(api_key: &str) -> anyhow::Result<()> {
  let config_path = config_file_path();
  println!("Saving to {}", format!("{:?}", config_path).cyan());

  let prefix = config_path.parent().unwrap();
  std::fs::create_dir_all(prefix)?;

  let mut file = std::fs::File::create(&config_path)?;
  let encrypted = encrypt(api_key.as_bytes(), encryption_password(), nonce()?)?;
  file.write_all(&*encrypted)
    .with_context(|| format!("Could not save API key to {:?}", config_path))?;
  Ok(())
}

/// Reads API key from environment variables
fn read_api_key_from_env() -> anyhow::Result<String> {
  match std::env::var("OPENAI_API_KEY") {
    Ok(key) => is_key_empty(key),
    Err(_) => Err(anyhow::anyhow!("OPENAI_API_KEY variable is not set correctly!"))
  }
}

/// Verifies key string is not empty and trims it
fn is_key_empty(key: String) -> anyhow::Result<String> {
  if key.trim().is_empty() {
    return Err(anyhow::anyhow!("API Key is empty!"))
  } else {
    return Ok(key.trim().to_string());
  }
}

/// Gets the config directory path
fn config_dir_path<'a>() -> std::path::PathBuf {
  directories::ProjectDirs::from("com", "pgpt", "pgpt")
    .unwrap()
    .config_dir()
    .to_path_buf()
}

/// Gets the config file path
fn config_file_path() -> std::path::PathBuf {
  config_dir_path().join("key.enc")
}

/// Gets the plaintext encryption password
fn encryption_password() -> String {
  return format!("{}_{}", whoami::username(), "pgpt_a1b2c3d4e5f6g7h8");
}