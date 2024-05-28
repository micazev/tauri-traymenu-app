use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_tray_menu(user_name: &str) -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            "user-name".to_string(),
            user_name.to_string(),
        ))
        .add_item(CustomMenuItem::new(
            "status".to_string(), 
            "Status: Registrando",
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        // Adicione outros itens do menu aqui
        .add_item(CustomMenuItem::new(
            "previsto".to_string(),
            "Previsto",
        ))
        .add_item(CustomMenuItem::new(
            "iniciar".to_string(),
            "Iniciar",
        ))
        // Adicione outros itens do menu aqui
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            "almoco".to_string(),
            "Iniciar almoço",
        ))
        .add_item(CustomMenuItem::new(
            "relatorio".to_string(),
            "Relatório gerencial",
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            "sobre".to_string(),
            "Sobre",
        ))
}
