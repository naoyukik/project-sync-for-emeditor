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
    BOOL::from(true)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn OnCommand(_hwnd: windows::Win32::Foundation::HWND) {
    // Placeholder for EmEditor plugin command
}
