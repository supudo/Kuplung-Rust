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
mod fractals;
mod shadertoy;
mod utils;

fn main() -> eframe::Result {
  kuplung::app::main()
}