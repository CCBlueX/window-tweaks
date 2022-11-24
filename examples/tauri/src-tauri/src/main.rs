// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_blur, apply_acrylic, apply_rounded_corners, NSVisualEffectMaterial};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      #[cfg(target_os = "macos")]
      apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

      #[cfg(target_os = "windows")]
      apply_blur(&window)
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

      //apply_rounded_corners(&window)
      //  .expect("Unsupported platform! 'apply_rounded_corners' is only supported on Windows 11");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
