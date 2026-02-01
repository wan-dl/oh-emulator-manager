use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct IOSSimulator {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub os_version: String,
    pub status: String,
}

#[tauri::command]
pub async fn list_ios_simulators() -> Result<Vec<IOSSimulator>, String> {
    #[cfg(not(target_os = "macos"))]
    {
        return Err("iOS simulators are only available on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("xcrun")
            .args(&["simctl", "list", "devices", "--json"])
            .output()
            .map_err(|e| format!("Failed to execute xcrun command: {}", e))?;

        if !output.status.success() {
            return Err("Failed to list iOS simulators".to_string());
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let mut simulators = Vec::new();

        if let Some(devices) = json["devices"].as_object() {
            for (runtime, device_list) in devices {
                if let Some(devices_array) = device_list.as_array() {
                    for device in devices_array {
                        if let (Some(udid), Some(name), Some(state)) = (
                            device["udid"].as_str(),
                            device["name"].as_str(),
                            device["state"].as_str(),
                        ) {
                            let status = if state == "Booted" {
                                "running"
                            } else {
                                "stopped"
                            };

                            simulators.push(IOSSimulator {
                                id: udid.to_string(),
                                name: name.to_string(),
                                device_type: name.to_string(),
                                os_version: runtime.replace("com.apple.CoreSimulator.SimRuntime.", ""),
                                status: status.to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(simulators)
    }
}

#[tauri::command]
pub async fn start_ios_simulator(id: String) -> Result<(), String> {
    #[cfg(not(target_os = "macos"))]
    {
        return Err("iOS simulators are only available on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("xcrun")
            .args(&["simctl", "boot", &id])
            .output()
            .map_err(|e| format!("Failed to boot simulator: {}", e))?;

        Command::new("open")
            .args(&["-a", "Simulator", "--args", "-CurrentDeviceUDID", &id])
            .spawn()
            .map_err(|e| format!("Failed to open Simulator app: {}", e))?;

        Ok(())
    }
}

#[tauri::command]
pub async fn stop_ios_simulator(id: String) -> Result<(), String> {
    #[cfg(not(target_os = "macos"))]
    {
        return Err("iOS simulators are only available on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("xcrun")
            .args(&["simctl", "shutdown", &id])
            .output()
            .map_err(|e| format!("Failed to shutdown simulator: {}", e))?;

        Ok(())
    }
}

#[tauri::command]
pub async fn delete_ios_simulator(id: String) -> Result<(), String> {
    #[cfg(not(target_os = "macos"))]
    {
        return Err("iOS simulators are only available on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("xcrun")
            .args(&["simctl", "delete", &id])
            .output()
            .map_err(|e| format!("Failed to delete simulator: {}", e))?;

        Ok(())
    }
}

#[tauri::command]
pub async fn wipe_ios_data(id: String) -> Result<(), String> {
    #[cfg(not(target_os = "macos"))]
    {
        return Err("iOS simulators are only available on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("xcrun")
            .args(&["simctl", "erase", &id])
            .output()
            .map_err(|e| format!("Failed to erase simulator: {}", e))?;

        Ok(())
    }
}

#[tauri::command]
pub async fn screenshot_ios(id: String) -> Result<String, String> {
    #[cfg(not(target_os = "macos"))]
    {
        return Err("iOS simulators are only available on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("screenshot_{}_{}.png", id, timestamp);
        let path = std::env::temp_dir().join(&filename);

        Command::new("xcrun")
            .args(&["simctl", "io", &id, "screenshot", path.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to take screenshot: {}", e))?;

        Ok(filename)
    }
}
