pub mod config;
pub mod encryption;

use reqwest::blocking::{ Client, Response };
use reqwest::header::{ HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT, AUTHORIZATION };
use serde::Deserialize;

const COMPLETION_URL: &'static str = "https://api.openai.com/v1/chat/completions";

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct CompletionResponse {
  choices: Vec<Choice>,
  created: i64,
  id: String,
  model: String,
  object: String,
  usage: Usage
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Choice {
  finish_reason: String,
  index: i32,
  message: Message,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Message {
  content: String,
  role: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Usage {
  completion_tokens: i32,
  prompt_tokens: i32,
  total_tokens: i32,
}

fn create_client(api_key: &str) -> anyhow::Result<Client> {
  let auth_val = format!("Bearer {}", api_key);
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
  headers.insert(USER_AGENT, HeaderValue::from_static("pgpt/0.1.0"));
  headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_val)?);
  let client = Client::builder()
    .default_headers(headers)
    .build()?;
  Ok(client)
}

/// Makes the ChatGPT request and returns the response
fn query_gpt(args: &config::CLIArgs, config: &config::Config) -> anyhow::Result<CompletionResponse> {
  let client = create_client(&config.api_key)?;
  let body = serde_json::json!({
    "model": args.model.api_model(),
    "messages": [
      {
        "role": "user",
        "content": &args.query
      }
    ]
  });
  let response: Response = client.post(COMPLETION_URL)
    .json(&body)
    .send()?;
  if response.status().is_success() {
    let completion: CompletionResponse = response.json()?;
    return Ok(completion);
  } else {
    let mut err: serde_json::Value = response.json()?;
    let message = err["error"]["message"].take();
    return Err(anyhow::anyhow!(message));
  }
}

/// Calculates the total cost for the query and response
fn calculate_cost(model: &config::Model, usage: &Usage) -> f64 {
  let cost = ((usage.prompt_tokens as f64) * model.prompt_cost()) + ((usage.completion_tokens as f64) * model.completion_cost());
  return cost;
}

pub fn run(args: &config::CLIArgs, config: &config::Config) -> anyhow::Result<()> {
  let response = query_gpt(args, config)?;
  println!("{}", response.choices[0].message.content);
  println!("");
  println!("Cost: ${}", calculate_cost(&args.model, &response.usage));
  Ok(())
}
