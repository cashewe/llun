use include_dir::{include_dir, Dir};

pub static DEFAULT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/data/");
pub static RULES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/data/rules");
pub static PROMPT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/data/prompts");
pub const DEFAULT_RULES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/data/default_rules.txt"));