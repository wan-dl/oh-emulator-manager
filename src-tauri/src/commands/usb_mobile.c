//
//  usb_mobile.c
//  跨平台移动设备探测库 (Windows + macOS)
//
//  扫描 USB 设备，识别 Android / iOS / 鸿蒙设备
//  作为静态库链接到 Rust，通过 FFI 调用
//

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

// ============================================================
// 平台头文件
// ============================================================

#ifdef _WIN32
    #include <windows.h>
    #include <initguid.h>
    #include <setupapi.h>
    #include <cfgmgr32.h>
#elif defined(__APPLE__)
    #include <dirent.h>
    #include <sys/stat.h>
    #include <pwd.h>
    #include <unistd.h>
    #include <CoreFoundation/CoreFoundation.h>
    #include <IOKit/IOKitLib.h>
    #include <IOKit/usb/IOUSBLib.h>
#else
    #error "Unsupported platform. Only Windows and macOS are supported."
#endif

// ============================================================
// 通用常量
// ============================================================

// Android 厂商 VID
static const unsigned short ANDROID_VENDORS[] = {
    0x18D1,  // Google
    0x04E8,  // Samsung
    0x12D1,  // Huawei
    0x2717,  // Xiaomi
    0x2A70   // OnePlus
};
#define ANDROID_VENDOR_COUNT 5

// Apple 厂商 VID
#define APPLE_VENDOR_ID 0x05AC

// 华为 VID（Android 和鸿蒙共用）
#define HUAWEI_VENDOR_ID 0x12D1

// iOS 真机 PID
static const unsigned short IOS_PRODUCT_IDS[] = {
    0x12A8,  // Normal USB mux
    0x12A7,  // Recovery
    0x12AB,  // DFU
    0x12AD   // Diagnostics
};
#define IOS_PRODUCT_ID_COUNT 4

#define MAX_DEVICES 64

// ============================================================
// 导出的数据结构（与 Rust FFI 对应）
// ============================================================

typedef enum {
    DEV_ANDROID = 0,
    DEV_IOS = 1,
    DEV_HARMONY = 2
} DeviceType;

typedef struct {
    DeviceType type;
    char name[256];
    char serial[256];
    char brand[256];
    char vendor_id[8];   // "0xXXXX"
    char product_id[8];  // "0xXXXX"
    bool usb_debugging;  // Android / 鸿蒙
    bool trusted;        // iOS
} CDeviceInfo;

// ============================================================
// 跨平台字符串比较封装
// ============================================================

static int portable_strcasecmp(const char *a, const char *b) {
#ifdef _WIN32
    return _stricmp(a, b);
#else
    return strcasecmp(a, b);
#endif
}

static int portable_strncasecmp(const char *a, const char *b, size_t n) {
#ifdef _WIN32
    return _strnicmp(a, b, n);
#else
    return strncasecmp(a, b, n);
#endif
}

// ============================================================
// 通用工具函数
// ============================================================

static char* stristr(const char *haystack, const char *needle) {
    if (!haystack || !needle) return NULL;
    size_t needle_len = strlen(needle);
    if (needle_len == 0) return (char*)haystack;
    while (*haystack) {
        if (portable_strncasecmp(haystack, needle, needle_len) == 0)
            return (char*)haystack;
        haystack++;
    }
    return NULL;
}

static bool str_lower_contains(const char *haystack, const char *needle) {
    return stristr(haystack, needle) != NULL;
}

static bool is_android_vendor(unsigned short vid) {
    for (int i = 0; i < ANDROID_VENDOR_COUNT; i++) {
        if (ANDROID_VENDORS[i] == vid) return true;
    }
    return false;
}

static bool is_ios_product(unsigned short pid) {
    for (int i = 0; i < IOS_PRODUCT_ID_COUNT; i++) {
        if (IOS_PRODUCT_IDS[i] == pid) return true;
    }
    return false;
}

// ============================================================
// Windows 平台实现
// ============================================================

#ifdef _WIN32

DEFINE_GUID(GUID_DEVINTERFACE_USB_DEVICE,
    0xA5DCBF10, 0x6530, 0x11D2,
    0x90, 0x1F, 0x00, 0xC0, 0x4F, 0xB9, 0x51, 0xED);

static bool parse_vid_pid(const char *hardware_id, unsigned short *vid, unsigned short *pid) {
    const char *v = stristr(hardware_id, "VID_");
    const char *p = stristr(hardware_id, "PID_");
    if (!v || !p) return false;
    *vid = (unsigned short)strtoul(v + 4, NULL, 16);
    *pid = (unsigned short)strtoul(p + 4, NULL, 16);
    return true;
}

static bool multi_sz_contains(const char *buf, DWORD buf_size, const char *needle) {
    const char *p = buf;
    const char *end = buf + buf_size;
    while (p < end && *p) {
        if (stristr(p, needle)) return true;
        p += strlen(p) + 1;
    }
    return false;
}

static bool check_adb_interface(DEVINST dev_inst) {
    DEVINST child;
    CONFIGRET cr = CM_Get_Child(&child, dev_inst, 0);

    while (cr == CR_SUCCESS) {
        char compat_ids[4096] = {0};
        ULONG buf_size = sizeof(compat_ids);

        if (CM_Get_DevNode_Registry_PropertyA(child, CM_DRP_COMPATIBLEIDS,
                NULL, compat_ids, &buf_size, 0) == CR_SUCCESS) {
            if (multi_sz_contains(compat_ids, buf_size, "Class_ff&SubClass_42&Prot_01")) {
                return true;
            }
        }

        buf_size = sizeof(compat_ids);
        memset(compat_ids, 0, sizeof(compat_ids));
        if (CM_Get_DevNode_Registry_PropertyA(child, CM_DRP_HARDWAREID,
                NULL, compat_ids, &buf_size, 0) == CR_SUCCESS) {
            if (multi_sz_contains(compat_ids, buf_size, "Class_ff&SubClass_42&Prot_01")) {
                return true;
            }
        }

        cr = CM_Get_Sibling(&child, child, 0);
    }
    return false;
}

static bool check_ios_trusted(DEVINST dev_inst) {
    DEVINST child;
    CONFIGRET cr = CM_Get_Child(&child, dev_inst, 0);

    while (cr == CR_SUCCESS) {
        char compat_ids[4096] = {0};
        ULONG buf_size = sizeof(compat_ids);

        if (CM_Get_DevNode_Registry_PropertyA(child, CM_DRP_COMPATIBLEIDS,
                NULL, compat_ids, &buf_size, 0) == CR_SUCCESS) {
            if (multi_sz_contains(compat_ids, buf_size, "Class_06&SubClass_01&Prot_01")) {
                return true;
            }
        }

        buf_size = sizeof(compat_ids);
        memset(compat_ids, 0, sizeof(compat_ids));
        if (CM_Get_DevNode_Registry_PropertyA(child, CM_DRP_HARDWAREID,
                NULL, compat_ids, &buf_size, 0) == CR_SUCCESS) {
            if (multi_sz_contains(compat_ids, buf_size, "Class_06&SubClass_01&Prot_01")) {
                return true;
            }
        }

        cr = CM_Get_Sibling(&child, child, 0);
    }
    return false;
}

// 导出函数：扫描 USB 设备，返回设备数量
int c_scan_usb_devices(CDeviceInfo *devices, int max_devices) {
    int count = 0;

    HDEVINFO dev_info = SetupDiGetClassDevsA(
        &GUID_DEVINTERFACE_USB_DEVICE,
        NULL, NULL,
        DIGCF_PRESENT | DIGCF_DEVICEINTERFACE
    );

    if (dev_info == INVALID_HANDLE_VALUE) return 0;

    SP_DEVINFO_DATA dev_data;
    dev_data.cbSize = sizeof(SP_DEVINFO_DATA);

    for (DWORD i = 0; SetupDiEnumDeviceInfo(dev_info, i, &dev_data); i++) {
        if (count >= max_devices) break;

        char hardware_id[512] = {0};
        if (!SetupDiGetDeviceRegistryPropertyA(dev_info, &dev_data,
                SPDRP_HARDWAREID, NULL,
                (PBYTE)hardware_id, sizeof(hardware_id), NULL)) {
            continue;
        }

        unsigned short vid = 0, pid = 0;
        if (!parse_vid_pid(hardware_id, &vid, &pid)) continue;

        bool android = is_android_vendor(vid);
        bool ios = (vid == APPLE_VENDOR_ID && is_ios_product(pid));
        if (!android && !ios) continue;

        CDeviceInfo *dev = &devices[count];
        memset(dev, 0, sizeof(CDeviceInfo));

        snprintf(dev->vendor_id, sizeof(dev->vendor_id), "0x%04X", vid);
        snprintf(dev->product_id, sizeof(dev->product_id), "0x%04X", pid);

        char buf[256] = {0};
        if (SetupDiGetDeviceRegistryPropertyA(dev_info, &dev_data,
                SPDRP_FRIENDLYNAME, NULL,
                (PBYTE)buf, sizeof(buf), NULL)) {
            strncpy(dev->name, buf, sizeof(dev->name) - 1);
        } else if (SetupDiGetDeviceRegistryPropertyA(dev_info, &dev_data,
                SPDRP_DEVICEDESC, NULL,
                (PBYTE)buf, sizeof(buf), NULL)) {
            strncpy(dev->name, buf, sizeof(dev->name) - 1);
        } else {
            strcpy(dev->name, "Unknown");
        }

        memset(buf, 0, sizeof(buf));
        if (SetupDiGetDeviceRegistryPropertyA(dev_info, &dev_data,
                SPDRP_MFG, NULL,
                (PBYTE)buf, sizeof(buf), NULL)) {
            strncpy(dev->brand, buf, sizeof(dev->brand) - 1);
        } else {
            strcpy(dev->brand, "Unknown");
        }

        char instance_id[512] = {0};
        if (SetupDiGetDeviceInstanceIdA(dev_info, &dev_data,
                instance_id, sizeof(instance_id), NULL)) {
            char *last_sep = strrchr(instance_id, '\\');
            if (last_sep && *(last_sep + 1)) {
                strncpy(dev->serial, last_sep + 1, sizeof(dev->serial) - 1);
            } else {
                strcpy(dev->serial, "Unknown");
            }
        } else {
            strcpy(dev->serial, "Unknown");
        }

        if (android) {
            if (vid == HUAWEI_VENDOR_ID &&
                (str_lower_contains(dev->brand, "hisilicon") ||
                 str_lower_contains(dev->name, "hdc"))) {
                dev->type = DEV_HARMONY;
            } else {
                dev->type = DEV_ANDROID;
            }
            dev->usb_debugging = check_adb_interface(dev_data.DevInst);
        } else {
            dev->type = DEV_IOS;
            strncpy(dev->brand, "Apple", sizeof(dev->brand) - 1);
            dev->trusted = check_ios_trusted(dev_data.DevInst);
        }

        count++;
    }

    SetupDiDestroyDeviceInfoList(dev_info);
    return count;
}

#endif // _WIN32

// ============================================================
// macOS 平台实现
// ============================================================

#ifdef __APPLE__

static bool get_usb_property_string(io_object_t device, CFStringRef key, char *buf, size_t buf_size) {
    CFTypeRef prop = IORegistryEntryCreateCFProperty(device, key, kCFAllocatorDefault, 0);
    if (!prop) return false;

    bool ok = false;
    if (CFGetTypeID(prop) == CFStringGetTypeID()) {
        ok = CFStringGetCString((CFStringRef)prop, buf, (CFIndex)buf_size, kCFStringEncodingUTF8);
    }
    CFRelease(prop);
    return ok;
}

static bool get_usb_property_int(io_object_t device, CFStringRef key, int *value) {
    CFTypeRef prop = IORegistryEntryCreateCFProperty(device, key, kCFAllocatorDefault, 0);
    if (!prop) return false;

    bool ok = false;
    if (CFGetTypeID(prop) == CFNumberGetTypeID()) {
        ok = CFNumberGetValue((CFNumberRef)prop, kCFNumberIntType, value);
    }
    CFRelease(prop);
    return ok;
}

static bool check_interface_match(io_object_t device, int target_class, int target_subclass, int target_protocol) {
    io_iterator_t iterator = 0;
    if (IORegistryEntryCreateIterator(device, kIOServicePlane,
            kIORegistryIterateRecursively, &iterator) != KERN_SUCCESS) {
        return false;
    }

    io_object_t entry;
    bool found = false;

    while ((entry = IOIteratorNext(iterator)) != 0) {
        int cls = -1, sub = -1, pro = -1;
        get_usb_property_int(entry, CFSTR(kUSBInterfaceClass), &cls);
        get_usb_property_int(entry, CFSTR(kUSBInterfaceSubClass), &sub);
        get_usb_property_int(entry, CFSTR(kUSBInterfaceProtocol), &pro);

        if (cls == target_class && sub == target_subclass && pro == target_protocol) {
            IOObjectRelease(entry);
            found = true;
            break;
        }

        IOObjectRelease(entry);
    }

    IOObjectRelease(iterator);
    return found;
}

static bool check_adb_interface(io_object_t device) {
    return check_interface_match(device, 0xFF, 0x42, 0x01);
}

static bool check_ios_trusted(io_object_t device) {
    return check_interface_match(device, 6, 1, 1);
}

// 导出函数：扫描 USB 设备，返回设备数量
int c_scan_usb_devices(CDeviceInfo *devices, int max_devices) {
    CFMutableDictionaryRef match_dict = IOServiceMatching(kIOUSBDeviceClassName);
    if (!match_dict) return 0;

    io_iterator_t iterator = 0;
    if (IOServiceGetMatchingServices(kIOMasterPortDefault, match_dict, &iterator) != KERN_SUCCESS) {
        return 0;
    }

    int count = 0;
    io_object_t usb_device;

    while ((usb_device = IOIteratorNext(iterator)) != 0) {
        if (count >= max_devices) {
            IOObjectRelease(usb_device);
            break;
        }

        int vid_int = 0, pid_int = 0;
        get_usb_property_int(usb_device, CFSTR("idVendor"), &vid_int);
        get_usb_property_int(usb_device, CFSTR("idProduct"), &pid_int);
        unsigned short vid = (unsigned short)vid_int;
        unsigned short pid = (unsigned short)pid_int;

        bool android = is_android_vendor(vid);
        bool ios = (vid == APPLE_VENDOR_ID && is_ios_product(pid));

        if (!android && !ios) {
            IOObjectRelease(usb_device);
            continue;
        }

        CDeviceInfo *dev = &devices[count];
        memset(dev, 0, sizeof(CDeviceInfo));

        snprintf(dev->vendor_id, sizeof(dev->vendor_id), "0x%04X", vid);
        snprintf(dev->product_id, sizeof(dev->product_id), "0x%04X", pid);

        if (!get_usb_property_string(usb_device, CFSTR(kUSBProductString),
                dev->name, sizeof(dev->name))) {
            strcpy(dev->name, "Unknown");
        }

        if (!get_usb_property_string(usb_device, CFSTR(kUSBVendorString),
                dev->brand, sizeof(dev->brand))) {
            strcpy(dev->brand, "Unknown");
        }

        if (!get_usb_property_string(usb_device, CFSTR(kUSBSerialNumberString),
                dev->serial, sizeof(dev->serial))) {
            strcpy(dev->serial, "Unknown");
        }

        if (android) {
            if (vid == HUAWEI_VENDOR_ID &&
                (str_lower_contains(dev->brand, "hisilicon") ||
                 str_lower_contains(dev->name, "hdc"))) {
                dev->type = DEV_HARMONY;
            } else {
                dev->type = DEV_ANDROID;
            }
            dev->usb_debugging = check_adb_interface(usb_device);
        } else {
            dev->type = DEV_IOS;
            strncpy(dev->brand, "Apple", sizeof(dev->brand) - 1);
            dev->trusted = check_ios_trusted(usb_device);
        }

        count++;
        IOObjectRelease(usb_device);
    }

    IOObjectRelease(iterator);
    return count;
}

#endif // __APPLE__
