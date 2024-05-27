use clap::Parser;
use std::str::FromStr;

/// ChatGPT model
#[derive(Debug, Parser, Clone, serde::Serialize)]
pub enum Model {
    GPT3,
    GPT4,
    GPT4o,
}

impl FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-3" => Ok(Model::GPT3),
            "gpt-4" => Ok(Model::GPT4),
            "gpt-4o" => Ok(Model::GPT4o),
            _ => Err(format!("'{}' is not a valid model", s)),
        }
    }
}

impl Model {
    /// Returns the string representation of the model
    pub fn to_string(&self) -> String {
        match self {
            Self::GPT3 => String::from("gpt-3"),
            Self::GPT4 => String::from("gpt-4"),
            Self::GPT4o => String::from("gpt-4o"),
        }
    }

    /// Returns the model to use with the API
    pub fn api_model(&self) -> String {
        match self {
            Self::GPT3 => String::from("gpt-3.5-turbo"),
            Self::GPT4 => String::from("gpt-4-turbo"),
            Self::GPT4o => String::from("gpt-4o"),
        }
    }

    /// Returns the price ($) per token for the model input
    pub fn prompt_cost(&self) -> f64 {
        match self {
            Self::GPT3 => 0.5 / 1e6,
            Self::GPT4 => 10.0 / 1e6,
            Self::GPT4o => 5.0 / 1e6,
        }
    }

    /// Returns the price ($) per token for the model output
    pub fn completion_cost(&self) -> f64 {
        match self {
            Self::GPT3 => 1.5 / 1e6,
            Self::GPT4 => 30.0 / 1e6,
            Self::GPT4o => 15.0 / 1e6,
        }
    }
}
