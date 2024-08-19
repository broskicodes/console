use async_openai::types::ChatCompletionRequestMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "prompt_flavour", rename_all = "lowercase")]
pub enum PromptFlavour {
    #[serde(rename = "initial_goals")]
    #[sqlx(rename = "initial_goals")]
    InitialGoals,
}

impl PromptFlavour {
    pub fn prompt(&self) -> &str {
        match self {
            PromptFlavour::InitialGoals => "Your name is Buddy. You are a life coach. Your job is to determine what the user's goals are. Your priority should be short to medium term goals; things that the user can start working on immediately. Aim to understand the user's values and motivations as well.\n\nYou will ask questions until you are satisfied that you have a good understanding of the user. Once you are satisfied, you will output <<interigation_complete>> to signal the end of questions and a final message to the user between <final_message></final_message> tags.",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub chat_id: Uuid,
    pub model: String,
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub flavour: PromptFlavour,
}
