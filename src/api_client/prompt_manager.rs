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
    pub system_prompt_scan: String,
    pub system_prompt_consistency: String,
    pub user_prompt: String,
}

/// constructor for the struct
impl PromptManager {
    pub fn new(
        rules: &RuleSet,
        files: &FileSet,
        context: &Option<String>,
    ) -> Result<Self, PromptManagerError> {
        let system_prompt_scan = Self::load_system_prompt("system_prompt_scan.txt")?;
        let system_prompt_consistency = Self::load_system_prompt("system_prompt_consistency.txt")?;
        let user_prompt = Self::load_user_prompt(rules, files, context)?;

        Ok(Self {
            system_prompt_scan,
            system_prompt_consistency,
            user_prompt,
        })
    }

    /// load in and format the system prompt
    pub fn load_system_prompt(prompt_filename: &str) -> Result<String, PromptManagerError> {
        let schema = schema_for!(Response);
        let formatted_schema = serde_json::to_string_pretty(&schema)?;

        let prompt_template = PROMPT_DIR
            .get_file(prompt_filename)
            .ok_or_else(|| PromptManagerError::FileNotFound(prompt_filename.to_string()))?
            .contents_utf8()
            .ok_or_else(|| PromptManagerError::InvalidUtf8(prompt_filename.to_string()))?;

        let formatted_prompt = prompt_template.replace("{formatted_schema}", &formatted_schema);

        Ok(formatted_prompt)

    }

    /// load in and format the users prompt
    pub fn load_user_prompt(
        rules: &RuleSet,
        files: &FileSet,
        context: &Option<String>,
    ) -> Result<String, PromptManagerError> {
        let rules_string = rules.to_string();
        let files_string = files.to_string();
        let prompt_path = "user_prompt_scan.txt";

        let prompt_template = PROMPT_DIR
            .get_file(prompt_path)
            .ok_or_else(|| PromptManagerError::FileNotFound(prompt_path.to_string()))?
            .contents_utf8()
            .ok_or_else(|| PromptManagerError::InvalidUtf8(prompt_path.to_string()))?;

        let mut formatted_prompt = prompt_template
            .replace("{rules}", &rules_string)
            .replace("{files}", &files_string)
            .to_owned();

        if context.is_some() {
            let contextual_prompt: &str = "\nThe user has also supplied the following additional context: {context}";
            formatted_prompt.push_str(contextual_prompt);
        }

        Ok(formatted_prompt)
    }
}

impl fmt::Display for PromptManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "# System Prompt Scan\n{}\n\nUser Prompt Scan\n{}\n\nSystem Prompt Consistency\n{}",
            self.system_prompt_scan, self.user_prompt, self.system_prompt_consistency
        )
    }
}