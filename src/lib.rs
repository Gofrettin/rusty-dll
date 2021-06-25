use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH,
    DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
use winapi::um::consoleapi::AllocConsole;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPVOID, TRUE};

#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn DllMain(hinst_dll: HINSTANCE, fdw_reason: DWORD, lp_reserved: LPVOID) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(|| unsafe {
                main_thread();
            });
        }
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => { return FALSE; }
    }
    TRUE
}

unsafe fn main_thread() {
    AllocConsole();
    println!("Hello from #RustLang!");

    loop { }
}
