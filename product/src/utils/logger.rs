use log::{info, warn, error, debug};
use env_logger::Env;

/// Initialize the logger with default settings.
/// Logs messages at the `info` level or as specified in the `RUST_LOG` environment variable.
pub fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            use std::io::Write;
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(
                buf,
                "[{}] - [{}] - {}",
                timestamp,
                record.level(),
                record.args()
            )
        })
        .init();

    info!("Logger initialized with level: {}", log::max_level());
    debug!("Debugging enabled.");
}
