use std::fmt;

use serde::{Serialize, Deserialize};
use crate::data::{RULES_DIR};

#[derive(Debug, thiserror::Error)]
pub enum RuleError {
    #[error("Requested rule doesn't exist")]
    RuleNotFound(),
    #[error("Rule file cant be translated to UTF-8")]
    RuleNotDecodable(),
    #[error("Rule failed to be read from json {0}")]
    RuleReadError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExample {
    pub violation: String,
    pub better: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    #[serde(skip)]
    pub rule_code: String,
    pub name: String,
    pub description: String,
    pub risk_if_violated: String,
    #[serde(default)]
    pub examples: Vec<RuleExample>,
}

impl Rule {
    /// load rule from a rule json
    pub fn from_file(rule_code: String) -> Result<Self, RuleError> {
        let filename = format!("{}.json", rule_code);
        let file = RULES_DIR
            .get_file(&filename)
            .ok_or_else(|| RuleError::RuleNotFound())?;

        let contents = file
            .contents_utf8()
            .ok_or_else(|| RuleError::RuleNotDecodable())?;

        let mut rule: Rule = serde_json::from_str(&contents)?;
        rule.rule_code = rule_code;

        Ok(rule)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "## {} - {}", self.rule_code, self.name)?;
        writeln!(f, "*{}*", self.description)?;
        writeln!(f, "**Risk if violated:** {}", self.risk_if_violated)?;
        for example in &self.examples {
            writeln!(f, "- Violation: {}\n  Better: {}", example.violation, example.better)?;
        }
        Ok(())
    }
}