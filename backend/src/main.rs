#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chaos_dyn_wallpaper::run;

fn main() {
  run()
}
