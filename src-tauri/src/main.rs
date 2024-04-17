// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    window::{Effect, EffectsBuilder},
    LogicalPosition, Manager, PhysicalSize, Position, Size,
};
use windows_sys::Win32::{
    Foundation::{BOOL, HWND, RECT},
    UI::Shell::{SHAppBarMessage, ABE_TOP, ABM_NEW, ABM_REMOVE, ABM_SETPOS, APPBARDATA},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn register_app_bar(window_handle: HWND, register: bool) -> RECT {
    let mut abd = APPBARDATA {
        cbSize: 48, // layout height
        hWnd: window_handle,
        uCallbackMessage: 0,
        uEdge: ABE_TOP as u32,
        lParam: 0,
        rc: RECT {
            left: 0,
            top: 0,
            right: 3440,
            bottom: 40,
        },
    };

    unsafe {
        if register {
            SHAppBarMessage(ABM_NEW, &mut abd);
            SHAppBarMessage(ABM_SETPOS, &mut abd);
        } else {
            SHAppBarMessage(ABM_REMOVE, &mut abd);
        }
    }

    abd.rc
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window
                .set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();
            let hwnd = window.hwnd().unwrap().0 as HWND;

            let rc = register_app_bar(hwnd, true);

            window
                .set_size(Size::Physical(PhysicalSize {
                    width: (rc.right - rc.left) as u32,
                    height: (rc.bottom - rc.top) as u32,
                }))
                .unwrap();

            window
                .set_position(Position::Logical(LogicalPosition {
                    x: rc.left as f64,
                    y: rc.top as f64,
                }))
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
