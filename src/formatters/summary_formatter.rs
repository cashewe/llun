use std::collections::HashMap;

use crate::api_client::{DetectedIssue, Response};
use crate::formatters::{OutputFormatter, OutputFormatterError};

pub struct SummaryFormatter;

/// make use of the output formatter abstraction
impl OutputFormatter for SummaryFormatter {
    fn format(&self, response: &Response) -> Result<String, OutputFormatterError> {
        if response.detected_issues.is_empty() {
            return Ok("\x1b[32m✓ No issues detected\x1b[0m".to_string());
        }

        let mut output = String::new();
        let mut issues_by_file: HashMap<String, Vec<&DetectedIssue>> = HashMap::new();
        for issue in &response.detected_issues {
            issues_by_file
                .entry(issue.file_path.clone())
                .or_insert_with(Vec::new)
                .push(issue);
        }

        // Display each file's issues
        for (file_path, issues) in &issues_by_file {
            output.push_str(&format!(
                "\n\x1b[1;34m------------ {} ------------\x1b[0m\n\n",
                file_path
            ));

            for issue in issues {
                output.push_str(&format!(
                    "  \x1b[31m{}\x1b[0m: {}\n",
                    issue.rule_code, issue.name
                ));
                output.push_str(&format!("    {}\n", issue.brief_description));

                if !issue.code_snippet.trim().is_empty() {
                    output.push_str(&format!(
                        "    \x1b[90m{}\x1b[0m\n",
                        issue.code_snippet.trim()
                    ));
                }

                if !issue.suggested_alternative.trim().is_empty() {
                    output.push_str(&format!(
                        "    \x1b[33mSuggestion:\x1b[0m {}\n",
                        issue.suggested_alternative
                    ));
                }

                output.push('\n');
            }
        }

        let total_issues = response.detected_issues.len();
        let total_files = issues_by_file.len();

        output.push_str("\x1b[1m============ Results Summary ============\x1b[0m\n\n");
        output.push_str(&format!(
            "\x1b[1;31mDiscovered {} issues in {} files:\x1b[0m\n",
            total_issues, total_files
        ));

        let mut file_list: Vec<_> = issues_by_file.iter().collect();
        file_list.sort_by_key(|(path, _)| path.as_str());

        for (file_path, issues) in file_list {
            output.push_str(&format!("  * {} with {} issues\n", file_path, issues.len()));
        }

        output.push_str("\n\x1b[1m=========================================\x1b[0m\n");

        Ok(output)
    }
}
