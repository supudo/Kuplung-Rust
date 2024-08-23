use std::sync::Mutex;

static LOG_BUFFER: Mutex<String> = Mutex::new(String::new());

#[macro_export]
macro_rules! do_log {
  ($($arg:tt)*) => {{
    kuplung_logger::add_log(std::fmt::format(format_args!($($arg)*)).as_str());
  }}
}

pub fn add_log(message: &str) {
  log::info!("{}", message);
  LOG_BUFFER.lock().unwrap().push_str(format!("{}\n", message).as_str());
}

pub fn get_log() -> String {
  return LOG_BUFFER.lock().unwrap().to_string();
}

pub fn clear_log() {
  LOG_BUFFER.lock().unwrap().clear();
}


