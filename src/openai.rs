use openai_api_rs::v1::api::Client as OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, ChatCompletionResponse};
use openai_api_rs::v1::common::{GPT3_5_TURBO, GPT4};
use openai_api_rs::v1::error::APIError;
use std::env;

pub fn
gpt4_version () -> String
{
  GPT4.to_string()
}

pub fn
gpt35_version () -> String
{
  GPT3_5_TURBO.to_string()
}

pub fn
open_ai_api_client () -> OpenAIClient
{
  OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string())
}

pub fn
issue_open_ai_request (client: &OpenAIClient, model: String, prompt: &str) -> Result<ChatCompletionResponse, APIError>
{
  let req = ChatCompletionRequest::new(
    model,
    vec![chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from(prompt)),
        name: None,
    }],
  );

  client.chat_completion(req)
}