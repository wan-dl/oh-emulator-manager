fn main() {
    // Skip Windows resource generation for GNU toolchain to avoid path issues
    #[cfg(not(all(windows, target_env = "gnu")))]
    tauri_build::build();

    // 用 cc crate 编译 usb_mobile.c 并静态链接
    let mut build = cc::Build::new();
    build
        .file("src/commands/usb_mobile.c")
        .warnings(false);

    build.compile("usb_mobile");

    // 链接平台系统库
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=setupapi");
        println!("cargo:rustc-link-lib=cfgmgr32");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    println!("cargo:rerun-if-changed=src/commands/usb_mobile.c");
}
