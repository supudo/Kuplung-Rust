#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]

use app::KuplungApp;

mod app;

fn main() {
    KuplungApp();
}