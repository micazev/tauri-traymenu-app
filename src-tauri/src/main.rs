#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod tray_menu;
mod utils;

use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};

fn main() {
    let user_name = utils::get_logged_in_user_name(); // Obtendo o nome do usuário logado do módulo utils
    let tray_menu = tray_menu::create_tray_menu(&user_name);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested {
                api, ..
            } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn on_system_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle =
                app.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window =
                        app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(true) => {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        },
                        Ok(false) => {
                            window.show();
                            item_handle.set_title("Hide").unwrap();
                        },
                        Err(e) => unimplemented!("what kind of errors happen here?"),
                    }
                }
                "quit" => app.exit(0),
                _ => {}
            }
        }
        _ => {}
    }
}
