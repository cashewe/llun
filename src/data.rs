use include_dir::{Dir, include_dir};

pub const DEFAULT_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/data/default.toml"
));
pub static RULES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/data/rules");
pub static PROMPT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/data/prompts");
