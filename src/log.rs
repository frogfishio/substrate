use crate::config;
use chrono::Local; // For timestamps

pub fn log(topic: &str, message: &str) {
    let config = config::global_config(); // Access the global configuration
    if config.log_topics.contains(topic) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] [{}]: {}", timestamp, topic, message);
    }
}
