use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Response {
    pub detected_issues: Vec<DetectedIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DetectedIssue {
    pub rule_code: String,
    pub file_path: String,
    pub brief_description: String,
    pub explanation: String,
    pub suggested_alternative: String,
    pub code_snippet: String,
}
