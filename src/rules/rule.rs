use std::fmt;

use serde::{Serialize, Deserialize};

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
    /// load rule from a rule json string
    pub fn from_json_str(rule_code: String, contents: &str) -> Result<Self, RuleError> {
        let mut rule: Rule = serde_json::from_str(contents)?;
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