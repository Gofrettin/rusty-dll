use winapi::um::libloaderapi::FreeLibraryAndExitThread;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE,
    HINSTANCE, LPVOID, TRUE
};
use std::ptr::null_mut;

#[no_mangle]
unsafe extern "C" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _: LPVOID) -> BOOL
{
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            CreateThread(
                null_mut(),
                0,
                std::mem::transmute::<_, unsafe extern "system" fn(LPVOID) -> DWORD>(main_thread as usize).into(),
                hinst_dll as _,
                0,
                null_mut()
            );
        }
        // DLL_PROCESS_DETACH => {}
        // DLL_THREAD_ATTACH => {}
        // DLL_THREAD_DETACH => {}
        _ => { return FALSE; }
    }
    TRUE
}

unsafe extern "system" fn main_thread(lp_thread_parameter: LPVOID) -> u32 {
    // AllocConsole();
    println!("Hello from #RustLang!");

    // Put your code here


    // FreeConsole();
    FreeLibraryAndExitThread(lp_thread_parameter as _, 0);
    return 0;
}
