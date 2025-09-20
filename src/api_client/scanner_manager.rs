use std::collections::HashMap;
use tokio::try_join;
use crate::api_client::{AvailableScanner, OpenAiClientError, OpenAiPublicScanner, Response, Scanner, ScannerError};


#[derive(Debug, thiserror::Error)]
pub enum ScannerManagerError {
    #[error("Error in OpenAiClient")]
    OpenAiClientError(#[from] OpenAiClientError),
    #[error("Chosen scanner not found")]
    ScannerNotFound(),
    #[error("Error whilst scanning")]
    ScannerError(#[from] ScannerError)
}

pub struct ScannerManager {
    scanners: HashMap<AvailableScanner, Box<dyn Scanner>>,
}

impl ScannerManager {
    pub fn new() -> Result<Self, ScannerManagerError> {
        let mut scanners: HashMap<AvailableScanner, Box<dyn Scanner>> = HashMap::new();

        scanners.insert(AvailableScanner::OpenAiPublic, Box::new(OpenAiPublicScanner::new()?));

        Ok(Self{ scanners })
    }

    /// use your chosen scanner (its open ai isnt you normie)
    /// to perform a scan
    pub async fn run_scan(&self, system_prompt: &str, user_prompt: &str, model: String, scanner: AvailableScanner, production_mode: bool) -> Result<Response, ScannerManagerError> {
        let chosen_scanner = self.scanners
            .get(&scanner)
            .ok_or_else(ScannerManagerError::ScannerNotFound)?;

        if production_mode {
            let (response1, response2, response3) = try_join!(
                chosen_scanner.scan_files(system_prompt, user_prompt, model.clone()),
                chosen_scanner.scan_files(system_prompt, user_prompt, model.clone()),
                chosen_scanner.scan_files(system_prompt, user_prompt, model.clone())
            )?;

            let combined = self.combine_responses(vec![response1, response2, response3]);

            Ok( combined )
        } else{
            println!("Consider using production mode if reliably reproducible results are desired");
            Ok( chosen_scanner.scan_files(system_prompt, user_prompt, model).await? )
        }
    }

    /// merge many async responses into a single Response object
    fn combine_responses(&self, responses: Vec<Response>) -> Response {
        let mut all_issues = Vec::new();
        for response in responses {
            all_issues.extend(response.detected_issues);
        }
        Response { detected_issues: all_issues }
    }
}
