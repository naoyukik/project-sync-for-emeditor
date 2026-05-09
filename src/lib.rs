use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::core::BOOL;

static mut G_HINSTANCE: HMODULE = HMODULE(std::ptr::null_mut());

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    hinst_dll: HMODULE,
    fdw_reason: u32,
    lpv_reserved: *const std::ffi::c_void,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            G_HINSTANCE = hinst_dll;
        },
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    BOOL(1)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn OnCommand(_hwnd: windows::Win32::Foundation::HWND) {
    // Placeholder for EmEditor plugin command
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn QueryStatus(_hwnd: windows::Win32::Foundation::HWND, pb_checked: *mut BOOL) -> BOOL {
    if !pb_checked.is_null() {
        unsafe {
            *pb_checked = BOOL(0);
        }
    }
    BOOL(1)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn OnEvents(
    _hwnd: windows::Win32::Foundation::HWND,
    _event: u32,
    _w_param: windows::Win32::Foundation::WPARAM,
    _l_param: windows::Win32::Foundation::LPARAM,
) {
}

#[unsafe(no_mangle)]
pub extern "C" fn GetMenuTextID() -> u32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn GetStatusMessageID() -> u32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn GetBitmapID() -> u32 {
    0
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn PlugInProc(
    _hwnd: windows::Win32::Foundation::HWND,
    _msg: u32,
    _w_param: windows::Win32::Foundation::WPARAM,
    _l_param: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    windows::Win32::Foundation::LRESULT(0)
}
