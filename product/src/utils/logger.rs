use log::info;
use env_logger::Env;

pub fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Logger initialized successfully");
}
