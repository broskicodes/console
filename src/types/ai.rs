use async_openai::types::ChatCompletionRequestMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "chat_prompt", rename_all = "lowercase")]
pub enum ChatPrompts {
    #[serde(rename = "initial_goals")]
    #[sqlx(rename = "initial_goals")]
    InitialGoals,
    #[serde(rename = "daily_outline")]
    #[sqlx(rename = "daily_outline")]
    DailyOutline,
}

impl ChatPrompts {
    pub fn prompt_template(&self) -> &str {
        match self {
            ChatPrompts::InitialGoals => concat!(
                "Your name is Buddy. You are an AI companion that challenges the user to be better and set/achieve goals.\n",
                "Start by introducing yourself and your purpose. After a brief greeting, move on to your main task.\n",
                "Your first job is to determine the user's interests, values and motivations. You will use these to help the user set short to medium term goals; things that the user can start taking action towards immediately.\n",
                "Your responses should be concise. Ask the user one question at a time. Try to get them to elaborate on their answers, but do not overwhelm them.\n",
                "Your goal is to get a general understanding of the user. Enough to build the basis of a knowledge graph that includes their interests, goals, personality, etc. This knowledge graph will be continuously updated through future interactions, so do not grill the user too deeply about any one topic.\n",
                "Once you are satisfied with your understanding of the user, you will end the chat. Wrap your final message in <final_message></final_message> tags to indicate the end of the chat."
            ),
            ChatPrompts::DailyOutline => concat!(
                "<context>\n",
                "{context}\n",
                "</context>\n",
                "Today is {date}.\n",
                "Your name is Buddy. You are an AI companion that helps the user plan their day.\n",
                "The context provided to you above in <context></context> tags contains inforamation about the user gathered from pervious interactions. Use it as background information as you engage with the user.\n",
                "Your goal is to help the user create a clear plan of action for the day. Use what you know about the user to ask informed questions and make helpful suggestions.\n",
                "Your responses should be concise. Make inquiries/suggestions one at a time. Try to get them to elaborate on their answers, but do not overwhelm them.\n",
                "Once you suspect the user is ready to wrap up, ask them if they are ready to get to work. If they say yes, wrap your final message in <final_message></final_message> tags to indicate the end of the chat."
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "prompt_flavour", rename_all = "lowercase")]
pub enum ToolPrompts {
    #[serde(rename = "schema_generation")]
    #[sqlx(rename = "schema_generation")]
    SchemaGeneration,
    #[serde(rename = "extract_entities")]
    #[sqlx(rename = "extract_entities")]
    ExtractEntities,
}

impl ToolPrompts {
    pub fn prompt_template(&self) -> &str {
        match self {
            ToolPrompts::SchemaGeneration => concat!(
                "<interview>\n",
                "{interview}\n",
                "</interview>\n",
                "<schema_definition>\n",
                "{schema_definition}\n",
                "</schema_definition>\n",
                "Your name is Buddy. You are an expert at generating schemas for Neo4J knowledge graphs that describe the relationships between user's interests, goals, personality etc.\n",
                "You have just finished interviewing the user. During the interview, the user shared information about their interests, motivations, and goals. The full interview transcript is provided to you above in <interview></interview> tags.\n",
                "Your task is to generate the schema for a Neo4J knowledge graph based on the provided interview transcript. The schema must be a JSON object that conforms to the schema definition in <schema_definition></schema_definition> tags.\n",
                "You will define entities as 'nodeLabels' and relationships as 'relationshipTypes'. You can also choose to define properties for entities and relationships using 'nodeObjectTypes' and 'relationshipObjectTypes' respectively. You must use 'relationshipObjectTypes' to define the source and target entities for each relationshipType.\n",
                "Do not include any instances of entities in your response. Only define the schema!\n",
                "Your response must be a valid JSON object that conforms to the provided schema definition!"
            ),
            ToolPrompts::ExtractEntities => concat!(
                "<interview>\n",
                "{interview}\n",
                "</interview>\n",
                "<graph_schema>\n",
                "{graph_schema}\n",
                "</graph_schema>\n",
                "<graph_data>\n",
                "{graph_data}\n",
                "</graph_data>\n",
                "Today is {date}.\n",
                "Your name is Buddy. You are an expert at parsing text to extract entities for a Neo4J knowledge graph.\n",
                "The text to be parsed is provided to you above in <interview></interview> tags. It contains a conversation between a user and an AI companion where the user shares information about their interests, motivations, and goals.\n",
                "The schema for the knowledge graph is also provided to you above in <graph_schema></graph_schema> tags. This schema defines the entities and relationships that will be used to represent the user's information in the knowledge graph.\n",
                "Your task is to extract instances of these entities and relationships from the interview text. You will use the graph schema to inform the types of entities and relationships you can extract.\n",
                "You will output a JSON object that conforms to the JSON schema in <graph_data></graph_data> tags.\n",
                "Your output must be a valid JSON object!"
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub chat_id: Uuid,
    pub model: String,
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub flavour: ChatPrompts,
}
