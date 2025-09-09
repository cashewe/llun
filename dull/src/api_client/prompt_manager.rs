use std::fmt;

use serde_json;
use serde::{Serialize, Deserialize};
use schemars::schema_for;

use crate::rules::{RuleSet};
use crate::files::{FileSet};
use crate::data::{PROMPT_DIR};
use crate::api_client::Response;

/// errors that can occur in the prompt manager
#[derive(Debug, thiserror::Error)]
pub enum PromptManagerError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("File is not valid UTF-8: {0}")]
    InvalidUtf8(String),
    #[error("JSON parsing failed: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },
}

/// may or may not need to serialise this tbh...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptManager {
    pub system_prompt: String,
    pub user_prompt: String,
}

/// constructor for the struct
impl PromptManager {
    pub fn new(
        rules: &RuleSet,
        files: &FileSet,
    ) -> Result<Self, PromptManagerError> {
        let system_prompt = Self::load_system_prompt()?;
        let user_prompt = Self::load_user_prompt(rules, files)?;

        Ok(Self {
            system_prompt,
            user_prompt,
        })
    }

    /// load in and format the system prompt
    pub fn load_system_prompt() -> Result<String, PromptManagerError> {
        let schema = schema_for!(Response);
        let formatted_schema = serde_json::to_string_pretty(&schema)?;

        let prompt_template = PROMPT_DIR
            .get_file("system_prompt.txt")
            .ok_or_else(|| PromptManagerError::FileNotFound("system_prompt.txt".to_string()))?
            .contents_utf8()
            .ok_or_else(|| PromptManagerError::InvalidUtf8("system_prompt.txt".to_string()))?;

        let formatted_prompt = prompt_template.replace("{formatted_schema}", &formatted_schema);

        Ok(formatted_prompt)

    }

    /// load in and format the users prompt
    pub fn load_user_prompt(
        rules: &RuleSet,
        files: &FileSet,
    ) -> Result<String, PromptManagerError> {
        let rules_string = rules.to_string();
        let files_string = files.to_string();

        let prompt_template = PROMPT_DIR
            .get_file("user_prompt.txt")
            .ok_or_else(|| PromptManagerError::FileNotFound("user_prompt.txt".to_string()))?
            .contents_utf8()
            .ok_or_else(|| PromptManagerError::InvalidUtf8("user_prompt.txt".to_string()))?;

        let formatted_prompt = prompt_template
            .replace("{rules}", &rules_string)
            .replace("{files}", &files_string);

        Ok(formatted_prompt)
    }
}

impl fmt::Display for PromptManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "# System Prompt\n{}\n\nUser Prompt\n{}",
            self.system_prompt, self.user_prompt
        )
    }
}