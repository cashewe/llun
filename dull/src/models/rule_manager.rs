use std::collections::HashSet;
use crate::data::{DEFAULT_RULES, RULES_DIR};
use crate::models::Ruleset;


// claude suggested these custom errors
#[derive(Debug, thiserror::Error)]
pub enum RuleManagerError {
    #[error("Invalid rule name: {0}")]
    InvalidRule(String),
    #[error("Failed to load default rules: {0}")]
    DefaultRulesError(String),
    #[error("Failed to load ruleset: {0}")]
    RulesetLoadError(String),
    #[error("No rules available in directory")]
    NoRulesAvailable,

/// The cli / toml values that a user can use to control rules
#[derive(Debug, Default, Clone)]
pub struct RuleSelectionConfig {
    pub select: Vec<String>,
    pub extend_select: Vec<String>,
    pub ignore: Vec<String>,
}

#[derive(Debug, Default)]
pub struct RuleManager {
    default_rules: &'static str,
    valid_rules: HashSet<String>,
}

impl RuleManager {
    pub fn new() {
        let valid_rules = Self::get_valid_rules()?;

        if valid_rules.is_empty() {
            return Err(RuleManagerError::NoRulesAvailable);
        }

        Ok(Self {
            DEFAULT_RULES,
            valid_rules,
        })
    }

    /// get list of rules files from the rules folder
    pub fn get_valid_rules() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
        let valid_rules = RULES_DIR
            .files()
            .filter_map(|file| { // is it worth limiting this to only .jsons or do we just trust i wont break it?
                file
                    .path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|s| s.to_string())
            })
            .collect();

        if valid_rules.is_empty() {
            return Err(RuleManagerError::RulesetLoadError("No rules to load."))
        }

        Ok(valid_rules)
    }

    // load the default rules in from the txt file
    pub fn get_default_rules() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let default_rules = DEFAULT_RULES
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        if default_rules.is_empty() {
            return Err(RuleManagerError::DefaultRulesError("No default rules in file."))
        }
        
        Ok(default_rules)
    }

    /// get the final list of selected rules based on the inputs in the config
    pub fn finalise_selected_rules(&self, config: &RuleSelectionConfig) -> Result<Vec<String>, RuleManagerError> {
        let mut selected_rules = if config.select.is_empty() {
            self.get_default_rules()?
        } else {
            config.select.clone()
        };  // if select is a list with at least one value, use it, else default back
        selected_rules.extend(config.extend_select.clone());

        for rule in &selected_rules {
            if !self.valid_rules.contains(rule) {
                return Err(RuleManagerError::InvalidRule(rule.clone()));
            }
        }

        let finalised_rules: Vec<String> = selected_rules
                .into_iter()
                .filter(|rule| !ignore.contains(rule))
                .collect();
        
        Ok(finalised_rules)
    }

    /// load a ruleset based on provided config
    pub fn load_ruleset(&self, config: &RuleSelectionConfig) -> Result<Ruleset, RuleManagerError> {
        let finalised_rules = self.finalise_selected_rules(config)?;

        Ruleset::load_from_json(finalised_rules).map_err(|e| RuleManagerError::RulesetLoadError(e.to_string()))
    }

    /// load the ruleset object from cli commands
    pub fn load_from_cli(&self, select: Vec<String>, extend_select: Vec<String>, ignore: Vec<String>) -> Result<Ruleset, RuleManagerError> {
        let config = RuleSelectionConfig({
            select,
            extend_select,
            ignore,
        });

        self.load_ruleset(&config)?
    }

    // we will eventually need to extend this object to also load from toml, and a hybrid of cli and toml... itll get phat.
}