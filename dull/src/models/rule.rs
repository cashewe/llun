use std::fs;

use serde::{Serialize, Deserialize};
use data::{RULES_DIR};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_code: String,
    pub brief_description: String,
    pub long_description: String,
    pub example: String,
}

impl Rule {
    /// load rule from a rule json
    pub fn from_file(rule_code: String) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = RULES_DIR.as_ref().join(format!("{}.json", rule_code));
        let content = fs::read_to_string(&file_path)?;

        let mut rule: Rule = serde_json::from_str(&content)?;
        rule.code = rule_code

        Ok(rule)
    }

    /// create rule programmatically, for custom rules in the pyproject.toml file
    pub fn new(
        rule_code: String,
        brief_description: String,
        long_description: String,
        example: String,
    ) -> Self {
        Self {
            rule_code,
            brief_description,
            long_description,
            example,
        }
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