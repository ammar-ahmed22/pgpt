use crate::config::model::Model;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};

const COMPLETION_URL: &'static str = "https://api.openai.com/v1/chat/completions";

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct GPTResponse {
    pub choices: Vec<GPTChoice>,
    pub created: i64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: GPTUsage,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct GPTChoice {
    pub finish_reason: String,
    pub index: i32,
    pub message: GPTMessage,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct GPTUsage {
    pub completion_tokens: i32,
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}

impl GPTUsage {
    /// Calculates the total cost of the query
    ///
    /// ### Arguments
    /// - `model` - The model used for the query (*cost is dependent on model*)
    pub fn total_cost(&self, model: &Model) -> f64 {
        ((self.prompt_tokens as f64) * model.prompt_cost())
            + ((self.completion_tokens as f64) * model.completion_cost())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GPTRole {
    System,
    Assistant,
    User,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GPTMessage {
    pub role: GPTRole,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GPTQuery {
    pub model: String,
    pub messages: Vec<GPTMessage>,
}

pub struct GPTQueryBuilder {
    pub model: Option<Model>,
    pub messages: Vec<GPTMessage>,
}

impl GPTQueryBuilder {
    pub fn new() -> Self {
        return Self {
            model: None,
            messages: Vec::new(),
        };
    }

    /// Sets the model
    ///
    /// ### Arguments
    /// - `model` - The model to use in the query
    pub fn model(&mut self, model: &Model) -> &mut Self {
        self.model = Some(model.clone());
        self
    }

    /// Adds a message to the query
    ///
    /// ### Arguments
    /// - `role` - The role of the message (e.g. "user")
    /// - `content` - The content of the message
    pub fn message(&mut self, role: GPTRole, content: &str) -> &mut Self {
        let message = GPTMessage {
            role,
            content: content.to_string(),
        };
        self.messages.push(message);
        self
    }

    /// Builds the query
    pub fn build(&self) -> anyhow::Result<GPTQuery> {
        if self.model.is_none() {
            return Err(anyhow::anyhow!("Cannot build query without model!"));
        }

        if self.messages.is_empty() {
            return Err(anyhow::anyhow!("Cannot build query with no messages!"));
        }

        let model = self.model.clone().unwrap();
        let query = GPTQuery {
            model: model.api_model(),
            messages: self.messages.clone(),
        };
        Ok(query)
    }
}

impl GPTQuery {
    pub fn builder() -> GPTQueryBuilder {
        GPTQueryBuilder::new()
    }
}

pub struct GPTClient {
    http_client: Client,
}

impl GPTClient {
    fn create_http_client(api_key: &str) -> anyhow::Result<Client> {
        let auth_val = format!("Bearer {}", api_key);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("pgpt/0.1.0"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_val)?);
        let client = Client::builder().default_headers(headers).build()?;
        Ok(client)
    }

    pub fn new(api_key: &str) -> anyhow::Result<Self> {
        let http_client = Self::create_http_client(api_key)?;
        let gpt = Self { http_client };
        Ok(gpt)
    }

    /// Queries ChatGPT
    ///
    /// ### Arguments
    /// - `query` - The query to send to ChatGPT
    pub fn query(&self, gpt_query: &GPTQuery) -> anyhow::Result<GPTResponse> {
        let response: Response = self
            .http_client
            .post(COMPLETION_URL)
            .json(gpt_query)
            .send()?;

        if response.status().is_success() {
            let gpt_response: GPTResponse = response.json()?;
            return Ok(gpt_response);
        } else {
            let mut err: serde_json::Value = response.json()?;
            let message = err["error"]["message"].take();
            return Err(anyhow::anyhow!(message));
        }
    }
}
