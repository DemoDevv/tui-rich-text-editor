use std::{
    io::{Read, Stdin, Stdout, Write},
    os::windows::io::AsRawHandle,
};

use crate::graphics::{Drawable, lines::Line};

mod graphics;

type BOOL = i32;
type SHORT = i16;
type WORD = u16;
type DWORD = u32;
type LPDWORD = *mut DWORD;
type HANDLE = *mut std::ffi::c_void;

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
    dw_size: COORD,
    dw_cursor_position: COORD,
    w_attributes: WORD,
    sr_window: SMALL_RECT,
    dw_maximum_window_size: COORD,
}

impl Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        CONSOLE_SCREEN_BUFFER_INFO {
            dw_size: COORD { x: 0, y: 0 },
            dw_cursor_position: COORD { x: 0, y: 0 },
            w_attributes: 0,
            sr_window: SMALL_RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dw_maximum_window_size: COORD { x: 0, y: 0 },
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
const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
// Output options
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

// Virtual terminal sequences
const ESC: &str = "\x1b";
const CSI: &str = "\x1b[";

struct Terminal {
    screen: Screen,
    input: Stdin,
    input_handle: HANDLE,
    in_mode: u32,
    out_mode: u32,
    saved_in_mode: u32,
    saved_out_mode: u32,
}

impl Terminal {
    fn new(input: Stdin, output: Stdout) -> Self {
        let output_handle = output.as_raw_handle();
        let screen = Screen::new(output, output_handle);

        let input_handle = input.as_raw_handle();

        Terminal {
            screen,
            input,
            input_handle: input_handle,
            in_mode: 0,
            out_mode: 0,
            saved_in_mode: 0,
            saved_out_mode: 0,
        }
    }

    fn enable_raw_mode(&mut self) {
        unsafe {
            GetConsoleMode(self.input_handle, &mut self.in_mode);
            self.saved_in_mode = self.in_mode;
            // Disable echo input, line input, and processed input
            SetConsoleMode(
                self.input_handle,
                self.in_mode & !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT),
            );
        }
    }

    fn disable_raw_mode(&mut self) {
        unsafe {
            SetConsoleMode(self.input_handle, self.saved_in_mode);
        }
    }

    fn enable_virtual_terminal_processing(&mut self) {
        unsafe {
            GetConsoleMode(self.screen.handle, &mut self.out_mode);
            self.saved_out_mode = self.out_mode;
            // Enable virtual terminal processing
            SetConsoleMode(
                self.screen.handle,
                self.out_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            );
        }
    }

    fn disable_virtual_terminal_processing(&mut self) {
        unsafe {
            SetConsoleMode(self.screen.handle, self.saved_out_mode);
        }
    }

    fn enter_alternate_buffer(&mut self) -> Result<(), std::io::Error> {
        write!(self.screen.output, "{}{}", ESC, "[?1049h")?;
        self.screen.output.flush()
    }

    fn exit_alternate_buffer(&mut self) -> Result<(), std::io::Error> {
        write!(self.screen.output, "{}{}", CSI, "?1049l")?;
        self.screen.output.flush()
    }

    fn read_key(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.input.read(buffer)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.exit_alternate_buffer().unwrap();
        self.disable_virtual_terminal_processing();
        self.disable_raw_mode();
    }
}

struct Screen {
    output: Stdout,
    handle: HANDLE,
    screen_size: COORD,
}

impl Screen {
    fn new(output: Stdout, handle: HANDLE) -> Self {
        Screen {
            output,
            handle,
            screen_size: COORD { x: 0, y: 0 },
        }
    }

    fn get_window_size(&self) -> COORD {
        unsafe {
            let mut console_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::default();
            GetConsoleScreenBufferInfo(self.handle, &mut console_screen_buffer_info);
            COORD {
                x: console_screen_buffer_info.sr_window.right
                    - console_screen_buffer_info.sr_window.left
                    + 1,
                y: console_screen_buffer_info.sr_window.bottom
                    - console_screen_buffer_info.sr_window.top
                    + 1,
            }
        }
    }

    #[deprecated = "should use UTF-8 symbols"]
    fn enable_line_drawing(&mut self) -> Result<(), std::io::Error> {
        write!(self.output, "{}{}", ESC, "(0")?;
        self.output.flush()
    }

    #[deprecated = "should use UTF-8 symbols"]
    fn disable_line_drawing(&mut self) -> Result<(), std::io::Error> {
        write!(self.output, "{}{}", ESC, "(B")?;
        self.output.flush()
    }

    fn draw(&mut self, item: impl Drawable) -> Result<(), std::io::Error> {
        item.draw(&mut self.output)?;
        self.output.flush()
    }

    fn draw_at(&mut self, coord: COORD, item: impl Drawable) -> Result<(), std::io::Error> {
        self.move_cursor(coord)?;
        self.draw(item)
    }

    fn move_cursor(&mut self, coord: COORD) -> Result<(), std::io::Error> {
        write!(self.output, "{}[{};{}H", ESC, coord.y, coord.x)?;
        self.output.flush()
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = Terminal::new(std::io::stdin(), std::io::stdout());

    terminal.enable_raw_mode();
    terminal.enable_virtual_terminal_processing();
    terminal.enter_alternate_buffer()?;

    println!("Window size: {:?}", terminal.screen.get_window_size());

    for i in 1..11 {
        terminal
            .screen
            .draw_at(COORD { x: i, y: i }, Line::Intersection)?;
    }

    let mut buffer = [0u8; 32];
    while let Ok(n) = terminal.read_key(&mut buffer) {
        if n == 0 {
            break;
        }

        if buffer[0] == 17 {
            // Handle Ctrl+Q key press
            break;
        }

        if let Ok(string) = std::str::from_utf8(&buffer[..n]) {
            write!(terminal.screen.output, "{}", string)?;
            terminal.screen.output.flush()?;
        }
    }

    Ok(())
}
