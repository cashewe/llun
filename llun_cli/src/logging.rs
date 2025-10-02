use tracing_subscriber::{EnvFilter};

pub fn init_tracing(verbosity: u8) {
    let cli_level = match verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(cli_level));

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr) // keep stdout clean for json outputs
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}