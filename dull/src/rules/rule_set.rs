use std::fmt;
use super::rule::Rule;


#[derive(Debug, Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// load rule files by code
    pub fn load_from_json(rule_codes: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut collection = Self::new();

        for rule_code in rule_codes {
            match Rule::from_file(rule_code) {
                Ok(rule) => collection.add_rule(rule),
                Err(e) => eprintln!("Failed to load rule: {}", e),
            }
        }
        Ok(collection)
    }

    /// add a rule to the Vec
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}

/// we will (hopefully) use display to insert into a markdown message?
impl fmt::Display for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# Rules\n\n")?;

        for rule in &self.rules {
            write!(f, "\n---\n{}\n", rule)?;
        }
        Ok(())
    }
}

/// owned iteration, may want to implement borrowed itteration in future?
impl IntoIterator for RuleSet {
    type Item = Rule;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}