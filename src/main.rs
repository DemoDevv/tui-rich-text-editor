use std::{
    io::{Read, Write},
    os::windows::io::AsRawHandle,
};

type SHORT = i16;
type WORD = u16;
type DWORD = u32;
type LPDWORD = *mut DWORD;
type HANDLE = *mut std::ffi::c_void;
type BOOL = i32;

#[repr(C)]
#[derive(Debug)]
struct COORD {
    x: SHORT,
    y: SHORT,
}

#[repr(C)]
#[derive(Debug)]
struct SMALL_RECT {
    left: SHORT,
    top: SHORT,
    right: SHORT,
    bottom: SHORT,
}

#[repr(C)]
#[derive(Debug)]
struct CONSOLE_SCREEN_BUFFER_INFO {
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: WORD,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
}

impl Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD { x: 0, y: 0 },
            dwCursorPosition: COORD { x: 0, y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dwMaximumWindowSize: COORD { x: 0, y: 0 },
        }
    }
}

#[cfg(target_family = "windows")]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;

    fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
    );
}

// Input options
const ENABLE_ECHO_INPUT: u32 = 0x0004;
const ENABLE_LINE_INPUT: u32 = 0x0002;
// Output options
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

// Virtual terminal sequences
const ESC: &str = "\x1b";

fn main() -> std::io::Result<()> {
    let mut input = std::io::stdin();
    let in_handle = input.as_raw_handle();
    let mut in_mode = 0u32;

    let mut output = std::io::stdout();
    let out_handle = output.as_raw_handle();
    let mut out_mode = 0u32;

    let mut console_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::default();

    unsafe {
        GetConsoleMode(in_handle, &mut in_mode);
        // Disable echo input and line input
        SetConsoleMode(in_handle, in_mode & !ENABLE_ECHO_INPUT & !ENABLE_LINE_INPUT);

        GetConsoleMode(out_handle, &mut out_mode);
        // Enable virtual terminal processing
        SetConsoleMode(out_handle, out_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);

        GetConsoleScreenBufferInfo(out_handle, &mut console_screen_buffer_info);
    }

    println!("{:?}", console_screen_buffer_info);

    let mut buffer = [0u8; 32];
    while let Ok(n) = input.read(&mut buffer) {
        if n == 0 {
            break;
        }

        if let Ok(string) = std::str::from_utf8(&buffer[..n]) {
            print!("{}", string);
            output.flush()?;
        }
    }

    Ok(())
}
