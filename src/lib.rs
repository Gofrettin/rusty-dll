use winapi::um::consoleapi::AllocConsole;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPVOID, TRUE};

#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    lp_reserved: LPVOID) -> BOOL
{
    match fdw_reason {
        // Spawns new thread when attaching to the process instead of blocking DllMain
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(|| unsafe {
                main_thread();
            });
        }
        // DLL_PROCESS_DETACH => {}
        // DLL_THREAD_ATTACH => {}
        // DLL_THREAD_DETACH => {}
        _ => { return FALSE; }
    }
    TRUE
}

unsafe fn main_thread() {
    AllocConsole();
    println!("Hello from #RustLang!");

    // Put your code here

    loop { }
}
