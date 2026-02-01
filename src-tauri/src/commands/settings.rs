use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub language: String,
    pub theme: String,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub close_to_minimize: bool,
    pub android_home: String,
    pub deveco_home: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
            theme: "system".to_string(),
            auto_start: false,
            minimize_to_tray: true,
            close_to_minimize: true,
            android_home: String::new(),
            deveco_home: String::new(),
        }
    }
}

#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    // TODO: Load from database or config file
    let mut settings = Settings::default();

    // Try to get Android SDK path from environment
    if let Ok(android_home) = std::env::var("ANDROID_HOME") {
        settings.android_home = android_home;
    } else if let Ok(android_sdk_root) = std::env::var("ANDROID_SDK_ROOT") {
        settings.android_home = android_sdk_root;
    }

    // Try to get DevEco path from environment
    if let Ok(deveco_home) = std::env::var("DEVECO_SDK_HOME") {
        settings.deveco_home = deveco_home;
    }

    Ok(settings)
}

#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    // TODO: Save to database or config file
    println!("Saving settings: {:?}", settings);
    Ok(())
}
