#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  api::dialog::ask, http::ResponseBuilder, CustomMenuItem, Event, GlobalShortcutManager, Manager,
  SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl,
};

use mouse_rs::{types::keys::Keys, Mouse};

#[tauri::command]
fn trigger_click() {
  let mouse = Mouse::new();
  mouse.click(&Keys::LEFT).expect("Unable to press button");
  println!("I was invoked from JS!");
}

fn main() {
  let tray_menu1 = SystemTrayMenu::new().add_item(CustomMenuItem::new("exit_app", "Quit"));

  let tray = SystemTray::new().with_menu(tray_menu1);

  tauri::Builder::default()
    // .system_tray(tray)
    // .on_system_tray_event(move |app, event| match event {
    //   SystemTrayEvent::MenuItemClick { id, .. } => {
    //     let item_handle = app.tray_handle().get_item(&id);
    //     match id.as_str() {
    //       "exit_app" => {
    //         std::process::exit(0);
    //       },
    //       _ => (),
    //     }
    //   }
    //   _ => (),
    // })
    .invoke_handler(tauri::generate_handler![trigger_click])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
