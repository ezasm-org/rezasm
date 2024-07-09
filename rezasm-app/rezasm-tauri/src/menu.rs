use serde::Serialize;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowMenuEvent};

lazy_static::lazy_static! {
    /**
     * Represents the Tauri native menu
     */
    pub static ref MENU: Menu = Menu::new()
        .add_submenu(Submenu::new("File", Menu::new()
            .add_item(CustomMenuItem::new("save", "Save").accelerator("CmdOrCtrl+S"))
            .add_item(CustomMenuItem::new("open", "Open").accelerator("CmdOrCtrl+O"))
            .add_item(CustomMenuItem::new("new_file", "New File").accelerator("CmdOrCtrl+N"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::Quit)
        ))
        .add_submenu(Submenu::new("Edit", Menu::new()
            .add_item(CustomMenuItem::new("undo", "Undo").accelerator("CmdOrCtrl+Z"))
            .add_item(CustomMenuItem::new("redo", "Redo").accelerator("CmdOrCtrl+Y"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("cut", "Cut").accelerator("CmdOrCtrl+X"))
            .add_item(CustomMenuItem::new("copy", "Copy").accelerator("CmdOrCtrl+C"))
            .add_item(CustomMenuItem::new("paste", "Paste").accelerator("CmdOrCtrl+V"))
            .add_item(CustomMenuItem::new("delete", "Delete").accelerator("Delete"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("find", "Find").accelerator("CmdOrCtrl+F"))
        ));
}

/// Handles menu events
pub fn menu_event_handler<R: tauri::Runtime>(event: WindowMenuEvent<R>) {
    let name_emitters = [
        "cut", "find", "delete", "new_file", "open", "redo", "save", "undo",
    ];
    match event.menu_item_id() {
        s if name_emitters.contains(&s) => {
            let _ = event.window().emit_all(
                "menu_event",
                MenuEvent {
                    menu_item: s.into(),
                },
            );
        }
        s => panic!("Unhandled menu item {s}"),
    };
}

/// Represents the data communicated when a menu item's action has a front-end impact
/// 
/// # Fields
///
/// - menu_item: the name of the menu event
#[derive(Clone, Serialize)]
struct MenuEvent {
    pub menu_item: String,
}
