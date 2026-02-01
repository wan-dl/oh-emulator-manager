fn main() {
    // Skip Windows resource generation for GNU toolchain to avoid path issues
    #[cfg(not(all(windows, target_env = "gnu")))]
    tauri_build::build()
}
