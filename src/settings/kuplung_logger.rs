use std::sync::Mutex;
use chrono::Utc;


static LOG_BUFFER: Mutex<String> = Mutex::new(String::new());

#[macro_export]
macro_rules! do_log {
  ($($arg:tt)*) => {{
    kuplung_logger::add_log(std::fmt::format(format_args!($($arg)*)).as_str());
  }}
}

pub fn add_log(message: &str) {
  let str = format!("[{}] {}\n", Utc::now().format("%H:%M:%S.%f"), message);
  log::info!("{}" ,str);
  LOG_BUFFER.lock().unwrap().push_str(str.as_str());
}

pub fn get_log() -> String {
  return LOG_BUFFER.lock().unwrap().to_string();
}

pub fn clear_log() {
  LOG_BUFFER.lock().unwrap().clear();
}


