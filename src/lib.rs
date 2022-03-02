use std::time::Duration;

#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> isize;
    fn FreeConsole() -> isize;
    fn FreeLibraryAndExitThread(h_module: usize, exit_code: u32) -> !;
}

#[no_mangle]
unsafe extern "system" fn DllMain(hinst_dll: usize, fdw_reason: u32, _reserved: usize) -> isize {
    match fdw_reason {
        1 => {
            std::thread::spawn(move || {
                unsafe { main_thread(hinst_dll); }
            });
            1
        }
        _ => 0
    }
}

#[no_mangle]
unsafe extern "system" fn main_thread(lp_thread_parameter: usize) {
    AllocConsole();

    println!("Hello from #RustLang!");

    // Put your code here
    std::thread::sleep(Duration::new(1, 0));

    FreeConsole();
    FreeLibraryAndExitThread(lp_thread_parameter as _, 0);
}
