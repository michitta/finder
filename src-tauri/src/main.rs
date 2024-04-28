// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ptr, sync::RwLock};

use tauri::{
    async_runtime::spawn,
    window::{Effect, EffectsBuilder},
    Manager, PhysicalPosition, PhysicalSize, Position, Size,
};
use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    UI::{
        Shell::{
            SHAppBarMessage, ABE_BOTTOM, ABE_TOP, ABM_NEW, ABM_REMOVE, ABM_SETPOS, APPBARDATA,
        },
        WindowsAndMessaging::{FindWindowA, ShowWindow},
    },
};
use wineventhook::{raw_event, EventFilter, WindowEventHook};

use crate::window::get_window_name;

mod window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn register_app_bar(window_handle: HWND, register: bool, rc: RECT, u_edge: u32) -> RECT {
    let mut abd = APPBARDATA {
        cbSize: 48, // layout height
        hWnd: window_handle,
        uCallbackMessage: 0,
        uEdge: u_edge,
        lParam: 0,
        rc,
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
            let finder = app.get_webview_window("main").unwrap();
            let layout = app.get_webview_window("layout").unwrap();
            let dock = app.get_webview_window("dock").unwrap();

            let window_width = app.primary_monitor().unwrap().unwrap().size().width as i32;
            let window_height = app.primary_monitor().unwrap().unwrap().size().height as i32;

            // Configure layout
            layout
                .set_effects(EffectsBuilder::new().clear_effects().build())
                .unwrap();

            // Configure finder
            finder
                .set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();
            let finder_hwnd = finder.hwnd().unwrap().0 as HWND;
            let finder_rc = register_app_bar(
                finder_hwnd,
                true,
                RECT {
                    left: 0,
                    top: 0,
                    right: window_width,
                    bottom: 40,
                },
                ABE_TOP as u32,
            );
            finder
                .set_size(Size::Physical(PhysicalSize {
                    width: (finder_rc.right - finder_rc.left) as u32,
                    height: (finder_rc.bottom - finder_rc.top) as u32,
                }))
                .unwrap();

            finder
                .set_position(Position::Physical(PhysicalPosition { x: 0, y: 0 }))
                .unwrap();

            let app_rw = RwLock::new(app.app_handle().clone());

            spawn(async move {
                let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

                let hook = WindowEventHook::hook(
                    EventFilter::default().event(raw_event::SYSTEM_FOREGROUND),
                    event_tx,
                )
                .await
                .unwrap();

                // Wait and print events
                while let Some(event) = event_rx.recv().await {
                    match event.window_handle() {
                        Some(hwnd) => {
                            let title = get_window_name(hwnd.as_ptr() as isize);
                            app_rw.read().unwrap().emit("window-name", title).unwrap()
                        }
                        None => {}
                    }
                }

                // Unhook the hook
                hook.unhook().await.unwrap();
            });

            // Configure dock
            dock.set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();
            let dock_hwnd = dock.hwnd().unwrap().0 as HWND;
            let dock_rc = register_app_bar(
                dock_hwnd,
                true,
                RECT {
                    left: 0,
                    top: 56,
                    right: 3440,
                    bottom: 0,
                },
                ABE_BOTTOM as u32,
            );
            dock.set_size(Size::Physical(PhysicalSize {
                width: (dock_rc.right - dock_rc.left) as u32,
                height: 56 + 8,
            }))
            .unwrap();

            dock.set_position(Position::Physical(PhysicalPosition {
                x: 0,
                y: window_height - 56 - 8,
            }))
            .unwrap();

            // Hide taskbar
            // unsafe {
            //     let taskbar = FindWindowA(b"Shell_TrayWnd\0".as_ptr() as *const u8, ptr::null());
            //     ShowWindow(taskbar, SW_SHOW);
            // }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
