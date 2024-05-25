pub mod config;
pub mod encryption;
pub mod gpt;

use anyhow::Context;
use colored::*;
use gpt::{GPTClient, GPTQuery, GPTResponse, GPTRole};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::sync::Arc;
use std::thread::JoinHandle;
use termimad::crossterm::style::Color::*;
use termimad::{rgb, MadSkin};

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
    "Circuits tingling...",
];

/// Creates the loading spinner
fn create_spinner() -> anyhow::Result<ProgressBar> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("🌍🌎🌏")
            .template("{msg} {spinner:.green}")
            .with_context(|| format!("Failed to set template"))?,
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

/// Handles logic for query command
///
/// ### Arguments
/// - `args` - An Arc value for the arguments from the CLI related to the query.
/// - `config` - An Arc value for the config
pub fn run_query(args: Arc<config::QueryArgs>, config: Arc<config::Config>) -> anyhow::Result<()> {
    // Cloning to be used in separate thread
    let args_clone = Arc::clone(&args);
    let config_clone = Arc::clone(&config);

    // Visuals
    let spinner = create_spinner()?;
    let skin = create_skin();

    let model = match &args.model {
        Some(model) => model.clone(),
        None => config.model.clone(),
    };
    let model_clone = Arc::new(model.clone());

    let handle: JoinHandle<anyhow::Result<GPTResponse>> = std::thread::spawn(move || {
        let gpt = GPTClient::new(&config_clone.api_key)?;
        let query = GPTQuery::builder()
            .model(&model_clone)
            .message(GPTRole::User, &args_clone.query)
            .build()?;
        let response = gpt.query(&query)?;
        Ok(response)
    });

    while !handle.is_finished() {
        spinner.tick();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    match handle.join() {
        Ok(result) => match result {
            Ok(response) => {
                println!(
                    "{}",
                    format!("Response from {}", response.model.magenta()).cyan()
                );
                println!("");
                println!(
                    "{}",
                    skin.term_text(response.choices[0].message.content.as_str())
                );
                println!("");
                if args.cost {
                    println!(
                        "{}: ${:.6}",
                        "Cost".green(),
                        response.usage.total_cost(&model)
                    );
                }
            }
            Err(e) => return Err(anyhow::anyhow!(e)),
        },
        Err(_) => return Err(anyhow::anyhow!("Thread failed to execute!")),
    }

    Ok(())
}
