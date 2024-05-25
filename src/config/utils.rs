use crate::config::{CacheValue, ConfigJSON};
use crate::encryption::{decrypt, encrypt, nonce};
use anyhow::Context;
use colored::*;
use std::io::{Read, Write};

/// Attempts to load the configuration json file.
pub fn load_config_file() -> anyhow::Result<ConfigJSON> {
    let config_path = config_file_path();
    let mut file = std::fs::File::open(&config_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let contents = String::from_utf8(buffer)?;
    let json: ConfigJSON = serde_json::from_str(contents.as_str())?;
    Ok(json)
}

/// Creates the config file with default values
pub fn register_config_file() -> anyhow::Result<ConfigJSON> {
    let config_path = config_file_path();
    let config = ConfigJSON {
        model: String::from("gpt-3"),
        cache_length: 5,
        context: 0,
    };
    println!(
        "Creating configuration file with default values at {}",
        format!("{:?}", config_path).cyan()
    );
    let json_str = serde_json::to_string(&config)?;
    let prefix = config_path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;
    let mut file = std::fs::File::create(&config_path)?;
    file.write(&json_str.as_bytes())
        .with_context(|| format!("Could not write config to {:?}", config_path))?;
    Ok(config)
}

pub fn save_config_file(config: &ConfigJSON) -> anyhow::Result<()> {
  let config_path = config_file_path();
  let mut file = std::fs::File::create(&config_path)
    .with_context(|| format!("Could not open config file"))?;
  let config_str = serde_json::to_string(config)?;
  file.write(&config_str.as_bytes())?;
  println!("{}", "Saved config successfully!".green());
  Ok(())
}

/// Attempts to load API key from env variable or config file
pub fn load_api_key() -> anyhow::Result<String> {
    if let Ok(key) = read_api_key_from_env() {
        return Ok(key);
    }

    let config_path = api_file_path();
    let mut buffer = Vec::new();
    let mut file = std::fs::File::open(&config_path)?;
    file.read_to_end(&mut buffer)?;
    let api_key_buf = decrypt(buffer, encryption_password())?;
    let api_key = String::from_utf8(api_key_buf)?;

    Ok(api_key)
}

/// Prompts user for API key and encrypts/writes it to file
pub fn register_api_key() -> anyhow::Result<String> {
    println!();
    println!("OpenAI API key not found!");
    println!();
    println!(
        "You need to enter an {} which will be encrypted and saved locally!",
        "OpenAI API key".cyan()
    );
    println!("You can create an API key at https://platform.openai.com/api-keys");
    println!();
    println!(
        "If you don't want to save it, you can pass your API key to the environment variable `{}`!",
        "OPENAI_API_KEY".cyan()
    );
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
        e => Err(anyhow::anyhow!("Received empty API key - {}", e)),
    }
    // Ok(String::new())
}

/// Attempts to load cache file
pub fn load_cache() -> anyhow::Result<Vec<CacheValue>> {
    let cache_path = cache_file_path();
    let mut file = std::fs::File::open(&cache_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let str_cache = String::from_utf8(buffer)?;
    let cache: Vec<CacheValue> = serde_json::from_str(&str_cache)?;
    Ok(cache)
}

/// Creates empty cache file if it doesn't exist
pub fn register_cache() -> anyhow::Result<()> {
    let cache_path = cache_file_path();
    if !cache_path.exists() {
        let prefix = cache_path.parent().unwrap();
        std::fs::create_dir_all(&prefix)?;
        let mut file = std::fs::File::create(cache_path)?;
        let empty: Vec<CacheValue> = vec![];
        let json_str = serde_json::to_string(&empty)?;
        file.write(&json_str.as_bytes())?;
        println!("{}", "Created cache file successfully!".green());
    }

    Ok(())
}

pub fn save_cache(cache: Vec<CacheValue>) -> anyhow::Result<()> {
    let cache_path = cache_file_path();
    let mut file = std::fs::File::create(&cache_path)?;
    let json_str = serde_json::to_string(&cache)?;
    file.write(&json_str.as_bytes())?;
    println!("{}", "Saved cache successfully!".green());
    Ok(())
}

pub fn save_api_key(api_key: &str) -> anyhow::Result<()> {
    let config_path = api_file_path();

    let prefix = config_path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;

    let mut file = std::fs::File::create(&config_path)?;
    let encrypted = encrypt(api_key.as_bytes(), encryption_password(), nonce()?)?;
    file.write_all(&*encrypted)
        .with_context(|| format!("Could not save API key to {:?}", config_path))?;
    println!(
        "{}",
        format!("Saved {} API key successfully!", "encrypted".cyan()).green()
    );
    Ok(())
}

/// Reads API key from environment variables
fn read_api_key_from_env() -> anyhow::Result<String> {
    match std::env::var("OPENAI_API_KEY") {
        Ok(key) => is_key_empty(key),
        Err(_) => Err(anyhow::anyhow!(
            "OPENAI_API_KEY variable is not set correctly!"
        )),
    }
}

/// Verifies key string is not empty and trims it
fn is_key_empty(key: String) -> anyhow::Result<String> {
    if key.trim().is_empty() {
        return Err(anyhow::anyhow!("API Key is empty!"));
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

/// Gets the api file path
pub fn api_file_path() -> std::path::PathBuf {
    config_dir_path().join("key.enc")
}

/// Gets the config file path
pub fn config_file_path() -> std::path::PathBuf {
    config_dir_path().join("./config.json")
}

/// Gets the cache file path
pub fn cache_file_path() -> std::path::PathBuf {
    config_dir_path().join("./cache.json")
}

/// Gets the plaintext encryption password
pub fn encryption_password() -> String {
    return format!("{}_{}", whoami::username(), "pgpt_a1b2c3d4e5f6g7h8");
}

/// Removes the local API key file.
pub fn clear_api_key() -> anyhow::Result<()> {
    let config_path = api_file_path();
    std::fs::remove_file(&config_path)?;
    println!(
        "Removed config file at {}",
        format!("{:?}", config_path).cyan()
    );
    Ok(())
}

/// Clears the local cache file
pub fn clear_cache() -> anyhow::Result<()> {
    let cache_path = cache_file_path();
    let mut file = std::fs::File::create(&cache_path)?;
    let empty: Vec<CacheValue> = Vec::new();
    let empty_str = serde_json::to_string(&empty)?;
    file.write(&empty_str.as_bytes())?;
    println!("{}", "Cleared cache successfully!".green());
    Ok(())
}
