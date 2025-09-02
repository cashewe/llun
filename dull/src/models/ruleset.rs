use super::rule::Rule;
use data::{RULES_DIR};


#[derive(Debug, Default)]
pub struct Ruleset {
    rules: Vec<Rule>,
}

impl Ruleset {
    pub fn new() -> Self {
        Self::default()
    }

    /// load rule files by code
    pub fn load_from_code(rule_codes: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut collection = Self::new();

        for rule_code in rule_codes {
            match Rule::from_file(rule_code, RULES_DIR) {
                Ok(rule) => collection.add_rule(rule),
                Err(e) => eprintln!("Failed to load rule {}: {}", rule_code, e),
            }
        }
        Ok(collection)
    }

    /// add a rule to the Vec
    pub fn add_rule(rule: Rule) {
        self.rules.push(rule);
    }
}

imple fmt::Display for Ruleset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# Rules\n\n")?;

        for rule in self.rules {
            write!(f, "\n---\n{}\n", rule)?;
        }
        Ok(())
    }
}