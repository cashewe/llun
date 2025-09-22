use std::fs::File;

use junit_report::{Report, TestCase, TestSuite, Duration};

use crate::formatters::{OutputFormatter, OutputFormatterError};
use crate::api_client::Response;

const OUTPUT_PATH: &str = "llun-results.xml";

pub struct JunitFormatter;

/// make use of the output formatter abstraction
impl OutputFormatter for JunitFormatter {
    fn format(&self, response: &Response) -> Result<String, OutputFormatterError> {
        let mut report = Report::new();
        let mut test_suite = TestSuite::new("Llun architectural review");
        
        if response.detected_issues.is_empty() {
            let test_case = TestCase::success("no_issues_detected", Duration::seconds(0));
            test_suite.add_testcase(test_case);
        } else {
            for (i, issue) in response.detected_issues.iter().enumerate() {
                // want unique test case names, so using rule code, file name (cleaned of non alphanumeric characters) and an integer
                let test_name = format!(
                    "{}_{}_{}", 
                    issue.file_path.replace(['/', '\\', '.'], "_"),
                    issue.rule_code,
                    i + 1
                );
                                
                let test_case = TestCase::failure(
                    &test_name,
                    Duration::seconds(0),
                    &issue.rule_code,
                    &issue.brief_description,
                );
                
                test_suite.add_testcase(test_case);
            }
        }
        
        report.add_testsuite(test_suite);

        let mut file = File::create(OUTPUT_PATH)
            .map_err(|e| OutputFormatterError::IoError(format!("Failed to create file: {}", e)))?;
        report.write_xml(&mut file).unwrap();
        
        Ok(format!("JUnit report created at: {}", OUTPUT_PATH))
    }
}