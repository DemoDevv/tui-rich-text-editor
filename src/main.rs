use std::{
    io::{Read, Write},
    os::windows::io::AsRawHandle,
};

type DWORD = u32;
type LPDWORD = *mut DWORD;
type HANDLE = *mut std::ffi::c_void;
type BOOL = i32;

#[cfg(target_family = "windows")]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

const ENABLE_ECHO_INPUT: u32 = 0x0004;
const ENABLE_LINE_INPUT: u32 = 0x0002;

fn main() -> std::io::Result<()> {
    let mut input = std::io::stdin();
    let in_handle = input.as_raw_handle();
    let mut in_mode = 0u32;

    let mut output = std::io::stdout();
    let out_handle = output.as_raw_handle();
    let mut out_mode = 0u32;

    unsafe {
        GetConsoleMode(in_handle, &mut in_mode);
        // Disable echo input and line input
        SetConsoleMode(in_handle, in_mode & !ENABLE_ECHO_INPUT & !ENABLE_LINE_INPUT);

        GetConsoleMode(out_handle, &mut out_mode);
    }

    let mut buffer = [0u8; 32];
    while let Ok(n) = input.read(&mut buffer) {
        if n == 0 {
            break;
        }

        if let Ok(string) = std::str::from_utf8(&buffer) {
            print!("{}", string);
            output.flush()?;
        }
    }

    Ok(())
}
