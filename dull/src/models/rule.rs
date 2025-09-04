use std::fmt;

use serde::{Serialize, Deserialize};
use crate::data::{RULES_DIR};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    #[serde(default)]
    pub rule_code: String,
    pub brief_description: String,
    pub long_description: String,
    pub example: String,
}

impl Rule {
    /// load rule from a rule json
    pub fn from_file(rule_code: String) -> Result<Self, Box<dyn std::error::Error>> {
        let filename = format!("{}.json", rule_code);
        let file = RULES_DIR
            .get_file(&filename)
            .ok_or_else(|| format!("File not found: {}", filename))?;

        let contents = file
            .contents_utf8()
            .ok_or_else(|| format!("File not valid UTF-8: {}", filename))?;

        let mut rule: Rule = serde_json::from_str(&contents)?;
        rule.rule_code = rule_code;

        Ok(rule)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "## {} - {}\n*{}*\n**Example**: {}",
            self.rule_code, self.brief_description, self.long_description, self.example
        )
    }
}