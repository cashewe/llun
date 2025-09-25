use std::collections::HashMap;
use crate::api_client::Response;

#[derive(Debug, thiserror::Error)]
pub enum PerFileIgnorerError {
    #[error("Format for per-file-ignores is wrong! {0}")]
    InvalidFormat(String),
    #[error("Per-file-ignore requires a file path: {0}")]
    NoPathProvided(String),
    #[error("Per-file-ignore requires at least one rule to ignore: {0}")]
    NoRulesProvided(String),
    #[error("Failed to parse response as JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Failed to extract json from response")]
    JsonCleaningError,
    #[error("Relevant secrets must be set in environment {0}")]
    MissingEnvVar(String),
}


#[derive(Debug, Clone)]
pub struct PerFileIgnorer {
    ignores: HashMap<String, Vec<String>>,
}

impl PerFileIgnorer {
    /// create an ignorer for the chosen ignore rules
    pub fn new(per_file_ignores: Vec<String>) -> Result<Self, PerFileIgnorerError> {
        let mut ignores = HashMap::new();
        
        for ignore_spec in per_file_ignores {
            Self::parse_ignore_spec(&ignore_spec, &mut ignores)?;
        }
        
        Ok(Self { ignores })
    }

    /// convert user requests into usable hashmap of files to ignore
    fn parse_ignore_spec(
        spec: &str, 
        ignores: &mut HashMap<String, Vec<String>>
    ) -> Result<(), PerFileIgnorerError> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts.len() != 2 {
            return Err(PerFileIgnorerError::InvalidFormat(format!("format: '{}'. Expected '<PATH>:<RULES>'", spec)));
        }
        
        let file_path = parts[0].trim();
        let rules_str = parts[1].trim();
        
        if file_path.is_empty() {
            return Err(PerFileIgnorerError::NoPathProvided("File path cannot be empty in per-file-ignore".to_string()));
        }
        
        let rules: Vec<String> = rules_str
            .split(',')
            .map(|rule| rule.trim().to_string())
            .filter(|rule| !rule.is_empty())
            .collect();
            
        if rules.is_empty() {
            return Err(PerFileIgnorerError::NoRulesProvided(format!("No rules specified for file '{}' in per-file-ignore", file_path)));
        }
        
        ignores.entry(file_path.to_string())
            .or_default()
            .extend(rules);
            
        Ok(())
    }

    /// if a given rule / filepath combo should be being ignored or not
    pub fn should_ignore(&self, file_path: &str, rule_code: &str) -> bool {
        if let Some(ignored_rules) = self.ignores.get(file_path)
            && ignored_rules.contains(&rule_code.to_string()) {
                return true;
            }
        
        for (ignore_path, ignored_rules) in &self.ignores {
            if Self::path_matches(file_path, ignore_path) && ignored_rules.contains(&rule_code.to_string()) {
                return true;
            }
        }
        
        false
    }

    /// find if the path is valid
    fn path_matches(file_path: &str, pattern: &str) -> bool {
        if file_path == pattern {
            return true;
        }
        
        if pattern.ends_with('/') && file_path.starts_with(pattern) {
            return true;
        }
        
        if file_path.starts_with(&format!("{}/", pattern)) {
            return true;
        }
        
        false
    }

    /// entryway to ignorance
    pub fn apply_ignores(&self, mut response: Response) -> Response {
        response.detected_issues.retain(|issue| !self.should_ignore(&issue.file_path, &issue.rule_code));
            
        response
    }

}