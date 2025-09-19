use std::collections::HashSet;
use crate::data::{RULES_DIR};
use crate::rules::{RuleSet, Rule};


// claude suggested these custom errors
#[derive(Debug, thiserror::Error)]
pub enum RuleManagerError {
    #[error("Invalid rule name: {0}")]
    InvalidRule(String),
    #[error("Failed to load default rules: {0}")]
    DefaultRulesError(String),
    #[error("Failed to load ruleset: {0}")]
    RuleSetLoadError(String),
    #[error("No rules available in directory")]
    NoRulesAvailable,
}

/// The cli / toml values that a user can use to control rules
#[derive(Debug, Default, Clone)]
pub struct RuleSelectionConfig {
    pub select: Vec<String>,
    pub extend_select: Vec<String>,
    pub ignore: Vec<String>,
}

#[derive(Debug, Default)]
pub struct RuleManager {
    valid_rules: HashSet<String>,
}

impl RuleManager {
    pub fn new() -> Result<Self, RuleManagerError> {
        let valid_rules = Self::get_valid_rules()?;

        Ok(Self {
            valid_rules,
        })
    }

    /// get list of rules files from the rules folder
    pub fn get_valid_rules() -> Result<HashSet<String>, RuleManagerError> {
        let mut valid_rules: HashSet<String> = RULES_DIR
            .files()
            .filter(|file| {
                file.path().extension().and_then(|s| s.to_str()) == Some("json")
            })
            .filter_map(|file| {
                file
                    .path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|s| s.to_string())
            })
            .collect();

        Self::add_user_defined_rules(&mut valid_rules)?;

        if valid_rules.is_empty() {
            return Err(RuleManagerError::RuleSetLoadError("No rules to load.".to_string()))
        }

        Ok(valid_rules)
    }

    /// add any user defined rules into the rules list
    /// and error out if the user defines rules we already defined
    /// you dont get to do that chum
    /// thats not *for* you
    pub fn add_user_defined_rules(valid_rules: &mut HashSet<String>) -> Result<(), RuleManagerError> {
        if let Ok(entries) = std::fs::read_dir("llun") {
            for entry in entries.flatten() {
                if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str())
                    && entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                        let rule_name = name.to_string();
                        if valid_rules.contains(&rule_name) {
                            return Err(RuleManagerError::RuleSetLoadError(
                                format!("User-defined rule '{}' conflicts with built-in rule", rule_name)
                            ));
                        }
                        valid_rules.insert(rule_name);
                    }
            }
        }
        Ok(())
    }

    /// get the final list of selected rules based on the inputs in the config
    pub fn finalise_selected_rules(&self, config: &RuleSelectionConfig) -> Result<Vec<String>, RuleManagerError> {
        let mut selected_rules = config.select.clone();
        selected_rules.extend(config.extend_select.clone());

        for rule in &selected_rules {
            if !self.valid_rules.contains(rule) {
                return Err(RuleManagerError::InvalidRule(rule.clone()));
            }
        }

        let finalised_rules: Vec<String> = selected_rules
                .into_iter()
                .filter(|rule| !config.ignore.contains(rule))
                .collect();
        
        Ok(finalised_rules)
    }

    /// load a ruleset based on provided config
    pub fn load_ruleset(&self, config: &RuleSelectionConfig) -> Result<RuleSet, RuleManagerError> {
        let finalised_rules = self.finalise_selected_rules(config)?;
        let mut collection = RuleSet::new();
        for rule_code in finalised_rules {
            let filename = format!("{}.json", rule_code);

            let contents = if let Ok(local_contents) = std::fs::read_to_string(format!("llun/{}", filename)) {
                local_contents
            } else if let Some(file) = RULES_DIR.get_file(&filename) {
                file.contents_utf8()
                    .ok_or_else(|| RuleManagerError::RuleSetLoadError(format!("Rule file not decodable: {}", rule_code)))?
                    .to_string()
            } else {
                return Err(RuleManagerError::RuleSetLoadError(format!("Rule file not found: {}", rule_code)));
            };

            match Rule::from_json_str(rule_code, &contents) {
                Ok(rule) => collection.add_rule(rule),
                Err(e) => return Err(RuleManagerError::RuleSetLoadError(e.to_string())),
            }
        }
        Ok( collection )
    }

    /// load the ruleset object from cli commands
    pub fn load_from_cli(&self, select: Vec<String>, extend_select: Vec<String>, ignore: Vec<String>) -> Result<RuleSet, RuleManagerError> {
        let config = RuleSelectionConfig {
            select,
            extend_select,
            ignore,
        };

        self.load_ruleset(&config)
    }
}