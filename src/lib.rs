use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::core::BOOL;

pub mod gui;

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
pub extern "system" fn OnCommand(hwnd: windows::Win32::Foundation::HWND) {
    use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
    use windows::core::w;

    unsafe {
        MessageBoxW(
            Some(hwnd),
            w!("Hello World from project-sync!"),
            w!("EmEditor Plugin"),
            MB_OK,
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn QueryStatus(
    _hwnd: windows::Win32::Foundation::HWND,
    pb_checked: *mut BOOL,
) -> BOOL {
    if !pb_checked.is_null() {
        // SAFETY: pb_checked は EmEditor プラグインインターフェースを通じて渡される
        // 有効なポインタであることを前提とする。
        unsafe {
            *pb_checked = BOOL(0);
        }
    }
    BOOL(1)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn OnEvents(
    _hwnd: windows::Win32::Foundation::HWND,
    _n_event: u32,
    _l_param: windows::Win32::Foundation::LPARAM,
) {
}

#[unsafe(no_mangle)]
pub extern "system" fn GetMenuTextID() -> u32 {
    0
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStatusMessageID() -> u32 {
    0
}

#[unsafe(no_mangle)]
pub extern "system" fn GetBitmapID() -> u32 {
    0
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn PlugInProc(
    _hwnd: windows::Win32::Foundation::HWND,
    _msg: u32,
    _w_param: windows::Win32::Foundation::WPARAM,
    _l_param: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    windows::Win32::Foundation::LRESULT(0)
}
