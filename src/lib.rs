pub mod config;
pub mod encryption;

use anyhow::Context;
use reqwest::blocking::{ Client, Response };
use reqwest::header::{ HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT, AUTHORIZATION };
use serde::Deserialize;
use colored::*;
use indicatif::{ ProgressBar, ProgressStyle };
use std::sync::Arc;
use rand::Rng;
use termimad::{ MadSkin, rgb };
use termimad::crossterm::style::Color::*;

const COMPLETION_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const LOADING_MESSAGES: [&'static str; 10] = [
  "Consulting neural network...",
  "Bribing data set...",
  "Thinking hard...",
  "Crunching 0s and 1s...",
  "Cooking up a response...",
  "AI is thinking...",
  "Turning AI gears...",
  "Consulting the AI crystal...",
  "Praying for your answer...",
  "Circuits tingling..."
];


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

/// Creates the HTTP request client
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
fn query_gpt(args: Arc<config::CLIArgs>, config: Arc<config::Config>) -> anyhow::Result<CompletionResponse> {
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

/// Creates the loading spinner
fn create_spinner() -> anyhow::Result<ProgressBar> {
  let spinner = ProgressBar::new_spinner();
  spinner.set_style(
    ProgressStyle::default_spinner()
      .tick_chars("🌍🌎🌏")
      .template("{msg} {spinner:.green}")
      .with_context(|| format!("Failed to set template"))? 
  );
  let mut rng = rand::thread_rng();
  let rand_idx = rng.gen_range(0..LOADING_MESSAGES.len());
  let rand_msg = LOADING_MESSAGES[rand_idx];
  spinner.set_message(rand_msg);
  Ok(spinner)
}

fn create_skin() -> MadSkin {
  let mut skin = MadSkin::default();
  skin.bold.set_fg(Yellow);
  skin.italic.set_fgbg(Yellow, rgb(30, 30, 40));

  return skin;
}
 
/// Runs the CLI
pub fn run(args: Arc<config::CLIArgs>, config: Arc<config::Config>) -> anyhow::Result<()> {
  let spinner = create_spinner()?;
  let skin = create_skin();
  let args_clone = Arc::clone(&args);
  let config_clone = Arc::clone(&config);
  let handle = std::thread::spawn(move || {
    let response = query_gpt(args_clone, config_clone).unwrap();
    response
  });

  while !handle.is_finished() {
    spinner.tick();
    std::thread::sleep(std::time::Duration::from_millis(200));
  }

  spinner.finish_and_clear();

  match handle.join() {
    Ok(response) => {
      // println!("response...");
      println!("{}", format!("Response from {}", args.model.api_model().magenta()).cyan());
      println!("");
      println!("{}", skin.term_text(response.choices[0].message.content.as_str()));
      println!("");
      if args.cost {
        println!("{}: ${:.6}", "Cost".green(), calculate_cost(&args.model, &response.usage));
      }
      Ok(())
    },
    Err(_) => {
      return Err(anyhow::anyhow!("Thread failed to execute!"))
    }
  }
}
