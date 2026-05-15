use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::core::BOOL;

pub mod application;
pub mod domain;
pub mod gui;
pub mod infra;

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
            init_logger();
        },
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    BOOL(1)
}

fn init_logger() {
    use simplelog::*;
    use std::fs::File;

    let mut log_path = std::env::temp_dir();
    log_path.push("project-sync-for-emeditor.log");

    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(log_path).unwrap(),
    );
    log::info!("Logger initialized.");
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn OnCommand(hwnd: windows::Win32::Foundation::HWND) {
    use crate::application::sync_project_workflow::SyncProjectWorkflow;
    use crate::gui::driver::emeditor_gui_driver::EmEditorGuiDriver;
    use crate::infra::driver::xml_io_driver::XmlIoDriver;
    use crate::infra::repository::project_xml_repository_impl::ProjectXmlRepositoryImpl;
    use windows::Win32::UI::WindowsAndMessaging::{
        MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MessageBoxW,
    };
    use windows::core::w;

    let gui_driver = EmEditorGuiDriver::new(hwnd);

    log::info!("OnCommand triggered.");

    let root_path = match gui_driver.get_active_file_path() {
        Ok(path) => {
            log::info!("Active file path resolved: {:?}", path);
            if path.exists() || path.is_absolute() {
                path.parent().map(|p| p.to_path_buf()).unwrap_or(path)
            } else {
                log::error!(
                    "Active file path is not a valid filesystem path: {:?}",
                    path
                );
                unsafe {
                    MessageBoxW(
                        Some(hwnd),
                        w!(
                            "The current file is not saved. Please save the file first to identify the project root."
                        ),
                        w!("Project Sync"),
                        MB_OK | MB_ICONERROR,
                    );
                }
                return;
            }
        }
        Err(e) => {
            log::error!("Path resolution failed. Error: {}", e);
            unsafe {
                MessageBoxW(
                    Some(hwnd),
                    w!("Please open a file first to identify the project root."),
                    w!("Project Sync"),
                    MB_OK | MB_ICONERROR,
                );
            }
            return;
        }
    };

    if root_path.as_os_str().is_empty() {
        log::error!("Final root_path is empty. Aborting sync.");
        return;
    }

    log::info!("Final root_path: {:?}", root_path);

    let driver = XmlIoDriver::new();
    let repository = ProjectXmlRepositoryImpl::new(driver);
    let workflow = SyncProjectWorkflow::new(repository);

    match workflow.run(root_path) {
        Ok(_) => {
            log::info!("Sync successful.");
            unsafe {
                MessageBoxW(
                    Some(hwnd),
                    w!("Project XML has been synchronized successfully."),
                    w!("Project Sync"),
                    MB_OK | MB_ICONINFORMATION,
                );
            }
        }
        Err(e) => unsafe {
            log::error!("Sync failed: {}", e);
            MessageBoxW(
                Some(hwnd),
                w!("Failed to synchronize project. Check logs for details."),
                w!("Project Sync Error"),
                MB_OK | MB_ICONERROR,
            );
        },
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
