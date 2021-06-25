# rusty-dll
Template project for creating Windows shared libraries (.dll)

# Quick start
You can either install `cargo-generate` tool with `cargo install cargo-generate` or clone this repository.
```
# Using `cargo-generate` tool
cargo generate https://github.com/sy1ntexx/rusty-dll

# Or clone the project itself
git clone https://github.com/sy1ntexx/rusty-dll
```

***Don't forget to rename package to your project name in*** `Cargo.toml`

# Usage
Write the code you want in `main_thread` function!
```rs
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
```
