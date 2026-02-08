use serde::Serialize;

const MAX_DEVICES: usize = 64;

const DEV_ANDROID: i32 = 0;
const DEV_IOS: i32 = 1;
const DEV_HARMONY: i32 = 2;

/// 与 C 侧 CDeviceInfo 结构体内存布局一致
#[repr(C)]
struct CDeviceInfo {
    device_type: i32,
    name: [u8; 256],
    serial: [u8; 256],
    brand: [u8; 256],
    vendor_id: [u8; 8],
    product_id: [u8; 8],
    usb_debugging: bool,
    trusted: bool,
}

extern "C" {
    fn c_scan_usb_devices(devices: *mut CDeviceInfo, max_devices: i32) -> i32;
}

/// 从 C 字符数组提取字符串，找到第一个 \0 截断
fn c_buf_to_bytes(buf: &[u8]) -> &[u8] {
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    &buf[..len]
}

/// Windows ANSI (GBK) → UTF-8，macOS/其他平台直接当 UTF-8
fn decode_c_string(buf: &[u8]) -> String {
    let bytes = c_buf_to_bytes(buf);
    if bytes.is_empty() {
        return String::new();
    }

    #[cfg(target_os = "windows")]
    {
        // Windows 的 A 系列 API 返回系统本地编码（中文系统是 GBK）
        let (decoded, _, _) = encoding_rs::GBK.decode(bytes);
        decoded.into_owned()
    }

    #[cfg(not(target_os = "windows"))]
    {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

/// 纯 ASCII 字段（VID/PID/serial），直接当 UTF-8 解析
fn decode_ascii_string(buf: &[u8]) -> String {
    let bytes = c_buf_to_bytes(buf);
    String::from_utf8_lossy(bytes).into_owned()
}

#[derive(Debug, Serialize, Clone)]
pub struct UsbDevice {
    #[serde(rename = "type")]
    pub device_type: String,
    pub name: String,
    pub serial: String,
    pub brand: String,
    pub vendor_id: String,
    pub product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usb_debugging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
}

#[tauri::command]
pub async fn list_usb_devices() -> Result<Vec<UsbDevice>, String> {
    let mut c_devices: Vec<CDeviceInfo> = Vec::with_capacity(MAX_DEVICES);
    for _ in 0..MAX_DEVICES {
        c_devices.push(unsafe { std::mem::zeroed() });
    }

    let count = unsafe { c_scan_usb_devices(c_devices.as_mut_ptr(), MAX_DEVICES as i32) };

    if count < 0 {
        return Err("USB scan failed".to_string());
    }

    let mut devices = Vec::new();
    for i in 0..count as usize {
        let c = &c_devices[i];
        let type_str = match c.device_type {
            DEV_ANDROID => "android",
            DEV_IOS => "ios",
            DEV_HARMONY => "harmony",
            _ => "unknown",
        };

        devices.push(UsbDevice {
            device_type: type_str.to_string(),
            name: decode_c_string(&c.name),
            serial: decode_ascii_string(&c.serial),
            brand: decode_c_string(&c.brand),
            vendor_id: decode_ascii_string(&c.vendor_id),
            product_id: decode_ascii_string(&c.product_id),
            usb_debugging: if c.device_type == DEV_IOS {
                None
            } else {
                Some(c.usb_debugging)
            },
            trusted: if c.device_type == DEV_IOS {
                Some(c.trusted)
            } else {
                None
            },
        });
    }

    Ok(devices)
}
