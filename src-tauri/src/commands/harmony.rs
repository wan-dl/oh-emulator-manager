use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct HarmonyEmulator {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub os_version: String,
    pub status: String,
}

#[tauri::command]
pub async fn list_harmony_emulators() -> Result<Vec<HarmonyEmulator>, String> {
    let output = Command::new("hdctool")
        .args(&["list", "targets"])
        .output()
        .map_err(|e| format!("Failed to execute hdctool command: {}", e))?;

    if !output.status.success() {
        return Err("Failed to list HarmonyOS emulators".to_string());
    }

    let device_list = String::from_utf8_lossy(&output.stdout);
    let mut emulators = Vec::new();

    for line in device_list.lines() {
        let line = line.trim();
        if !line.is_empty() && line.contains("emulator") {
            emulators.push(HarmonyEmulator {
                id: line.to_string(),
                name: line.to_string(),
                device_type: "HarmonyOS Device".to_string(),
                os_version: "HarmonyOS NEXT".to_string(),
                status: "stopped".to_string(),
            });
        }
    }

    Ok(emulators)
}

#[tauri::command]
pub async fn start_harmony_emulator(id: String) -> Result<(), String> {
    Command::new("hdctool")
        .args(&["start", &id])
        .spawn()
        .map_err(|e| format!("Failed to start emulator: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn stop_harmony_emulator(id: String) -> Result<(), String> {
    Command::new("hdctool")
        .args(&["stop", &id])
        .output()
        .map_err(|e| format!("Failed to stop emulator: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn screenshot_harmony(id: String) -> Result<String, String> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("screenshot_{}_{}.png", id, timestamp);
    let remote_path = "/data/local/tmp/screenshot.png";
    let local_path = std::env::temp_dir().join(&filename);

    Command::new("hdc")
        .args(&["-t", &id, "shell", "snapshot_display", "-f", remote_path])
        .output()
        .map_err(|e| format!("Failed to take screenshot: {}", e))?;

    Command::new("hdc")
        .args(&["-t", &id, "file", "recv", remote_path, local_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to download screenshot: {}", e))?;

    Ok(filename)
}
