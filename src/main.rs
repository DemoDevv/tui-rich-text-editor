use std::{
    io::{Read, Stdin, Stdout, Write},
    os::windows::io::AsRawHandle,
};

use crate::graphics::{Drawable, FrameBuffer, VirtualCursor, chars::Char, lines::Line};

mod graphics;
mod rope;

type BOOL = i32;
type SHORT = i16;
type WORD = u16;
type DWORD = u32;
type LPDWORD = *mut DWORD;
type HANDLE = *mut std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Drop the terminal, exiting the alternate buffer and disabling virtual terminal processing.
    fn drop(&mut self) {
        self.exit_alternate_buffer().unwrap();
        self.disable_virtual_terminal_processing();
        self.disable_raw_mode();
    }
}

struct Screen {
    output: Stdout,
    handle: HANDLE,
    cursor: VirtualCursor,
    frame_buffer: FrameBuffer,
    screen_size: COORD,
}

impl Screen {
    fn new(output: Stdout, handle: HANDLE) -> Self {
        let screen_size = Self::get_window_size_from_handle(handle);

        Screen {
            output,
            handle,
            cursor: VirtualCursor::default(),
            frame_buffer: FrameBuffer::new(screen_size.x, screen_size.y),
            screen_size,
        }
    }

    fn get_window_size_from_handle(handle: HANDLE) -> COORD {
        unsafe {
            let mut console_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::default();
            GetConsoleScreenBufferInfo(handle, &mut console_screen_buffer_info);
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

    fn draw_and_flush(&mut self, item: impl Drawable) -> Result<(), std::io::Error> {
        item.draw(&mut self.output)?;
        self.output.flush()
    }

    fn draw_at(&mut self, coord: COORD, item: impl Drawable) -> Result<(), std::io::Error> {
        self.move_cursor(coord)?;
        item.draw(&mut self.output)
    }

    fn move_cursor(&mut self, coord: COORD) -> Result<(), std::io::Error> {
        write!(self.output, "{}[{};{}H", ESC, coord.y + 1, coord.x + 1)
    }
}

impl Write for Screen {
    // note for me: we can register changes in hashmap or just a vector for tracking change and apply with O(n)
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let characters = std::str::from_utf8(buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        characters.chars().map(Char::from).for_each(|c| {
            if c.is_newline() {
                self.cursor
                    .set_position(0, (self.cursor.y + 1).min(self.screen_size.y - 1));
            } else if c.is_delete() {
                if self.cursor.x > 0 {
                    self.cursor.set_position(self.cursor.x - 1, self.cursor.y);
                }
                self.frame_buffer
                    .insert(Char::from(' '), self.cursor.x, self.cursor.y);
            } else {
                self.frame_buffer.insert(c, self.cursor.x, self.cursor.y);

                let new_x_pos = (self.cursor.x + 1) % self.screen_size.x;
                if self.cursor.x + 1 == self.screen_size.x {
                    self.cursor
                        .set_position(new_x_pos, (self.cursor.y + 1).min(self.screen_size.y - 1));
                } else {
                    self.cursor.set_position(new_x_pos, self.cursor.y);
                }
            }
        });

        Ok(characters.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        for (coord, cell) in self.frame_buffer.changes() {
            self.draw_at(coord, cell.character)?;
        }

        let final_pos = COORD {
            x: self.cursor.x,
            y: self.cursor.y,
        };
        self.move_cursor(final_pos)?;

        // apply the changes
        self.output.flush()?;
        // clear the changes
        self.frame_buffer.clear();

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = Terminal::new(std::io::stdin(), std::io::stdout());

    terminal.enable_raw_mode();
    terminal.enable_virtual_terminal_processing();
    terminal.enter_alternate_buffer()?;

    println!(
        "Window size: {:?}",
        Screen::get_window_size_from_handle(terminal.screen.handle)
    );

    for i in 1..11 {
        terminal
            .screen
            .draw_at(COORD { x: i, y: i }, Line::Intersection)?;
    }

    terminal.screen.output.flush()?;

    let mut buffer = [0u8; 32];
    while let Ok(n) = terminal.read_key(&mut buffer) {
        if n == 0 {
            break;
        }

        // Handle Ctrl+Q key press
        if buffer[0] == 17 {
            break;
        }

        terminal.screen.write(&buffer[..n])?;
        terminal.screen.flush()?;
    }

    Ok(())
}
