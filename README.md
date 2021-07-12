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

**First compile attempt can take significant amount of type due to compilation of `winapi` crate**

***Don't forget to rename package to your project name in*** `Cargo.toml`

`build/` folder contains batch scripts to quickly build your packet for `x64` or `x86` architecture.

# Usage
Write the code you want in `main_thread` function.
```rs
#[no_mangle]
unsafe extern "C" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lp_reserved: LPVOID) -> BOOL
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

```
