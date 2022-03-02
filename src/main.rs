#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]

mod app;
use app::kuplung_app;

fn main() {
    kuplung_app();
}