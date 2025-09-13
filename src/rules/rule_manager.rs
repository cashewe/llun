use std::collections::HashSet;
use crate::data::{RULES_DIR};
use crate::rules::RuleSet;


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
        let valid_rules: HashSet<String> = RULES_DIR
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

        if valid_rules.is_empty() {
            return Err(RuleManagerError::RuleSetLoadError("No rules to load.".to_string()))
        }

        Ok(valid_rules)
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

        RuleSet::load_from_json(finalised_rules).map_err(|e| RuleManagerError::RuleSetLoadError(e.to_string()))
    }

    /// load the ruleset object from cli commands
    pub fn load_from_cli(&self, select: Vec<String>, extend_select: Vec<String>, ignore: Vec<String>) -> Result<RuleSet, RuleManagerError> {
        let config = RuleSelectionConfig {
            select,
            extend_select,
            ignore,
        };

        Ok(self.load_ruleset(&config)?)
    }
}