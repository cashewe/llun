use anyhow::Result;
use crate::output_formatter::OutputFormatter;
use crate::api_client::Response;

pub struct AzureFormatter;

/// make use of the output formatter abstraction
impl OutputFormatter for AzureFormatter {
    fn format(&self, response: &Response) -> Result<String> {
        if response.detected_issues.is_empty() {
            return Ok("##[section]No architecture issues detected".to_string());
        }

        let mut output = String::new();
        
        for issue in &response.detected_issues {
            // Azure DevOps warning format
            output.push_str(&format!(
                "\n\n##vso[task.logissue type=warning]{}: Rule {} ({})\n{}\n{}",
                issue.file_path,
                issue.rule_code,
                issue.brief_description,
                issue.code_snippet,
                issue.explanation,
            ));
        }

        // Summary section
        output.push_str(&format!(
            "\n\n##vso[task.logissue type=info]Architecture Analysis Complete - {} issue(s) found\n",
            response.detected_issues.len()
        ));

        Ok(output)
    }
}