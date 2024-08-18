use async_openai::types::ChatCompletionRequestMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptFlavour {
    #[serde(rename = "life_coach")]
    LifeCoach,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub flavour: PromptFlavour,
}
