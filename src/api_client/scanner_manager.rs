use std::collections::HashMap;
use anyhow::Result;
use crate::api_client::{Scanner, AvailableScanner, Response, OpenAiPublicScanner};

pub struct ScannerManager {
    scanners: HashMap<AvailableScanner, Box<dyn Scanner>>,
}

impl ScannerManager {
    pub fn new() -> Result<Self> {
        let mut scanners: HashMap<AvailableScanner, Box<dyn Scanner>> = HashMap::new();

        scanners.insert(AvailableScanner::OpenAiPublic, Box::new(OpenAiPublicScanner::new()?));

        Ok(Self{ scanners })
    }

    /// use your chosen scanner (its open ai isnt you normie)
    /// to perform a scan
    pub async fn run_scan(&self, system_prompt: &String, user_prompt: &String, model: String, scanner: AvailableScanner) -> Result<Response> {
        let chosen_scanner = self.scanners
            .get(&scanner)
            .ok_or_else(|| anyhow::anyhow!("Scanner {:?} not found", scanner))?;
            
        chosen_scanner.scan_files(system_prompt, user_prompt, model).await
    }
}