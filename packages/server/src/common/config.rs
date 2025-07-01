pub struct Config;

impl Config {
    pub fn source_path() -> String {
        std::env::var("SOURCE_PATH").expect("SOURCE_PATH env not provided")
    }

    pub fn target_path() -> String {
        std::env::var("TARGET_PATH").expect("TARGET_PATH env not provided")
    }
}
