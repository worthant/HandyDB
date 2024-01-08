pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub fn log_platform_info(&self) {
        let compiled_for = if cfg!(target_os = "windows") {
            "x86_64-windows-msvc"
        } else if cfg!(target_os = "linux") {
            "x86_64-linux-gnu"
        } else {
            // Add ORM here for fun
            "unknown"
        };
    
        println!("Hello! This HandyDB release was compiled for {}.", compiled_for);
    }

    pub fn enable_logging(&self) {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
}