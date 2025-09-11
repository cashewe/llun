use std::fs;
use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    #[serde(default)]
    pub name: String,
    pub content: String,
}

impl File {
    /// load a file from a given path
    pub fn from_file(file_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&file_path)?;

        Ok(File { name: file_path, content })
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "## **{}**:\n\n {}",
            self.name, self.content,
        )
    }
}