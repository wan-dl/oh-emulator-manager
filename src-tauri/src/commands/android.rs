use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidEmulator {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub os_version: String,
    pub status: String,
}

#[tauri::command]
pub async fn list_android_emulators() -> Result<Vec<AndroidEmulator>, String> {
    let output = Command::new("emulator")
        .arg("-list-avds")
        .output()
        .map_err(|e| format!("Failed to execute emulator command: {}", e))?;

    if !output.status.success() {
        return Err("Failed to list Android emulators".to_string());
    }

    let avd_list = String::from_utf8_lossy(&output.stdout);
    let mut emulators = Vec::new();

    for line in avd_list.lines() {
        let name = line.trim();
        if !name.is_empty() {
            emulators.push(AndroidEmulator {
                id: name.to_string(),
                name: name.to_string(),
                device_type: "Android Device".to_string(),
                os_version: "Unknown".to_string(),
                status: "stopped".to_string(),
            });
        }
    }

    // Check running emulators
    if let Ok(adb_output) = Command::new("adb").arg("devices").output() {
        let devices = String::from_utf8_lossy(&adb_output.stdout);
        for emu in &mut emulators {
            if devices.contains(&emu.id) {
                emu.status = "running".to_string();
            }
        }
    }

    Ok(emulators)
}

#[tauri::command]
pub async fn start_android_emulator(id: String) -> Result<(), String> {
    Command::new("emulator")
        .arg("-avd")
        .arg(&id)
        .spawn()
        .map_err(|e| format!("Failed to start emulator: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn stop_android_emulator(id: String) -> Result<(), String> {
    let output = Command::new("adb")
        .args(&["devices"])
        .output()
        .map_err(|e| format!("Failed to get devices: {}", e))?;

    let devices = String::from_utf8_lossy(&output.stdout);
    
    for line in devices.lines() {
        if line.contains(&id) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(serial) = parts.first() {
                Command::new("adb")
                    .args(&["-s", serial, "emu", "kill"])
                    .output()
                    .map_err(|e| format!("Failed to stop emulator: {}", e))?;
                return Ok(());
            }
        }
    }

    Err("Emulator not found".to_string())
}

#[tauri::command]
pub async fn delete_android_emulator(id: String) -> Result<(), String> {
    Command::new("avdmanager")
        .args(&["delete", "avd", "-n", &id])
        .output()
        .map_err(|e| format!("Failed to delete emulator: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn wipe_android_data(id: String) -> Result<(), String> {
    Command::new("emulator")
        .args(&["-avd", &id, "-wipe-data"])
        .spawn()
        .map_err(|e| format!("Failed to wipe data: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn screenshot_android(id: String) -> Result<String, String> {
    let output = Command::new("adb")
        .args(&["devices"])
        .output()
        .map_err(|e| format!("Failed to get devices: {}", e))?;

    let devices = String::from_utf8_lossy(&output.stdout);
    
    for line in devices.lines() {
        if line.contains(&id) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(serial) = parts.first() {
                let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let filename = format!("screenshot_{}_{}.png", id, timestamp);
                
                Command::new("adb")
                    .args(&["-s", serial, "exec-out", "screencap", "-p"])
                    .output()
                    .map_err(|e| format!("Failed to take screenshot: {}", e))?;

                return Ok(filename);
            }
        }
    }

    Err("Emulator not found".to_string())
}
