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