// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Env, Manager};
use tauri_plugin_updater::UpdaterExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(desktop)]
            check_for_update(app.handle().clone())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(desktop)]
fn check_for_update(app_handle: AppHandle) -> tauri::Result<()> {
    #[cfg(target_os = "linux")]
    let updater_enabled = cfg!(dev) || app_handle.state::<Env>().appimage.is_some();
    #[cfg(not(target_os = "linux"))]
    let updater_enabled = true;
    if updater_enabled {
        tauri::async_runtime::spawn(async move {
            let updater = app_handle.updater();
            match updater {
                Ok(updater) => {
                    let response = updater.check().await;
                    match response {
                        Ok(update_option) => {
                            if let Some(update) = update_option {
                                println!("update available:\n\tdownload url: {}\n\tsignature: {}", update.download_url, update.signature);
                            } else {
                                println!("running latest version");
                            }
                        },
                        Err(e) => {
                            println!("updater failed to check: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("updater failed to build: {}", e);
                },
            }
        });
    } else {
        println!("updater not enabled");
    }
    Ok(())
}
