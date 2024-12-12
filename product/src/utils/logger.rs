use log::info;

pub fn init_logger() {
    env_logger::init();
    info!("Logger initialized");
}
