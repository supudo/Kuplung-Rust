#![cfg_attr(
  all(
    not(debug_assertions),
  ),
  windows_subsystem = "windows"
)]

mod kuplung;
mod settings;
mod rendering;
mod ui;

fn main() -> eframe::Result {
  kuplung::app::main()
}