// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod ability;

use std::{os::windows::process::CommandExt, process::Command};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
/// 执行cmd命令
fn execute_cmd_command(cmd_command: &str) -> String {
    // 关于设置Windows cmd https://github.com/tauri-apps/tauri/issues/10452
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let output = Command::new("cmd")
        .args(&["/C", cmd_command])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("failed");

    // 将输出转换为字符串
    let version = String::from_utf8_lossy(&output.stdout);
    return version.to_string();
}

fn main() {
    let context = tauri::generate_context!();

    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_cmd_command])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(ability::system_tray::menu()) // ✅ 将 `tauri.conf.json` 上配置的图标添加到系统托盘
        .on_system_tray_event(ability::system_tray::handle_system_tray_event); // ✅ 注册系统托盘事件处理程序

    builder
        .run(tauri::generate_context!()) 
        .expect("error while running tauri application");  
}
