use std::sync::Mutex;
use log::{error, info, warn};

static LOG_BUFFER: Mutex<String> = Mutex::new(String::new());

pub fn log_info(message: &str) {
  info!("{}", message);
  LOG_BUFFER.lock().unwrap().push_str(format!("{}\n", message).as_str());
}

#[allow(unused)]
pub fn log_warn(message: &str) {
  warn!("{}", message);
  LOG_BUFFER.lock().unwrap().push_str(message);
}

#[allow(unused)]
pub fn log_error(message: &str) {
  error!("{}", message);
  LOG_BUFFER.lock().unwrap().push_str(message);
}

pub fn get_info() -> String {
  return LOG_BUFFER.lock().unwrap().to_string();
}

#[allow(unused)]
pub fn get_warn() -> String {
  return LOG_BUFFER.lock().unwrap().to_string();
}

#[allow(unused)]
pub fn get_error() -> String {
  return LOG_BUFFER.lock().unwrap().to_string();
}

pub fn clear_log() {
  LOG_BUFFER.lock().unwrap().clear();
}
