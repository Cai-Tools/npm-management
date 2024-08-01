// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::api::process::restart;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn execute_cmd_command(cmd_command: &str) -> String {
    let output = Command::new("cmd")
        .args(&["/C", cmd_command])
        .output()
        .expect("failed");

    // 将输出转换为字符串
    let version = String::from_utf8_lossy(&output.stdout);
    return version.to_string();
}

#[tauri::command]
fn restart_tauri() {
    restart(&tauri::Env::default());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_cmd_command, restart_tauri])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
