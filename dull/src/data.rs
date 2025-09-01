use include_dir::{include_dir, Dir};

pub static RULES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/rules");
pub const DEFAULT_RULES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/default_rules.txt"));