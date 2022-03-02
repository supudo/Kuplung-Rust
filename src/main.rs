#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]

use app::kuplung_app;

mod app;

fn main() {
    kuplung_app();
}