use log::{info, warn, error};

pub fn init_logger() {
    env_logger::init();
    info!("Logger initialized");
}
