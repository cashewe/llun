use crate::api_client::{
    AvailableScanner, OpenAiClientError, OpenAiScanner, Response, Scanner, ScannerError,
};
use futures::future::try_join_all;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, thiserror::Error)]
pub enum ScannerManagerError {
    #[error("Error in OpenAiClient")]
    OpenAiClientError(#[from] OpenAiClientError),
    #[error("Chosen scanner not found")]
    ScannerNotFound(),
    #[error("Error whilst scanning")]
    ScannerError(#[from] ScannerError),
    #[error("No scanners available")]
    NoScannersAvailable,
}

pub struct ScannerManager {
    scanners: HashMap<AvailableScanner, Box<dyn Scanner>>,
}

impl ScannerManager {
    pub fn new() -> Result<Self, ScannerManagerError> {
        let mut scanners: HashMap<AvailableScanner, Box<dyn Scanner>> = HashMap::new();

        Self::try_register_scanner(&mut scanners, AvailableScanner::OpenAi, OpenAiScanner::new);
        Self::try_register_scanner(
            &mut scanners,
            AvailableScanner::AzureOpenAi,
            OpenAiScanner::new_azure,
        );

        if scanners.is_empty() {
            return Err(ScannerManagerError::NoScannersAvailable);
        }

        Ok(Self { scanners })
    }

    /// spent ages trying to find a way to register mappings -_-
    /// in the end both gpt and claude offered this as the solution
    /// supprisingly the only difference was claude having clearer var names?
    /// im sure this cant be the best method as its ugly as sin
    /// but for now, it works.
    fn try_register_scanner<F>(
        scanners: &mut HashMap<AvailableScanner, Box<dyn Scanner>>,
        scanner_type: AvailableScanner,
        constructor: F,
    ) where
        F: FnOnce() -> Result<OpenAiScanner, OpenAiClientError>,
    {
        match constructor() {
            Ok(scanner) => {
                scanners.insert(scanner_type, Box::new(scanner));
            }
            Err(e) => {
                debug!("Failed to initialize scanner: {}", e);
            }
        }
    }

    /// use your chosen scanner (its open ai isnt you normie)
    /// to perform a scan
    pub async fn run_scan(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        model: &str,
        consistency_prompt: &str,
        scanner: AvailableScanner,
        production_mode: bool,
    ) -> Result<Response, ScannerManagerError> {
        let chosen_scanner = self
            .scanners
            .get(&scanner)
            .ok_or_else(ScannerManagerError::ScannerNotFound)?;

        if production_mode {
            // maybe let the user configure 'n'?
            let futures =
                (0..5).map(|_| chosen_scanner.scan_files(system_prompt, user_prompt, model));
            let results = try_join_all(futures).await?;
            let combined = self.combine_responses(results);

            Ok(chosen_scanner
                .scan_files(
                    consistency_prompt,
                    &serde_json::to_string(&combined).unwrap(),
                    model,
                )
                .await?)
        } else {
            Ok(chosen_scanner
                .scan_files(system_prompt, user_prompt, model)
                .await?)
        }
    }

    /// merge many async responses into a single Response object
    fn combine_responses(&self, responses: Vec<Response>) -> Response {
        let mut all_issues = Vec::new();
        for response in responses {
            all_issues.extend(response.detected_issues);
        }
        Response {
            detected_issues: all_issues,
        }
    }
}
