use std::io;

use libc::{STDOUT_FILENO, ioctl, termios};

pub struct Terminal {
    pub width: u16,
    pub height: u16,

    original: termios,
}

impl Terminal {
    pub fn setup_raw() -> io::Result<Self> {
        let original = enable_raw_mode()?;
        let size = get_size()?;

        // Switch to alternate screen
        print!("\x1b[?1049h");
        // Hide the cursor
        print!("\x1b[?25l");
        // Reset formatting
        print!("\x1b[0m");

        Ok(Self {
            original,
            width: size.0,
            height: size.1,
        })
    }

    pub fn refresh_size(&mut self) -> io::Result<()> {
        let size = get_size()?;
        self.width = size.0;
        self.height = size.1;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Reset formatting
        print!("\x1b[0m");
        // Leave alternate screen
        print!("\x1b[?1049l");
        // Show cursor
        print!("\x1b[?25h");

        // Restore terminal attributes
        match disable_raw_mode(self.original) {
            Err(e) => {
                eprintln!("{e}");
            }
            _ => {}
        }
    }
}

pub fn get_size() -> io::Result<(u16, u16)> {
    let size = std::mem::MaybeUninit::<libc::winsize>::zeroed();
    let result = unsafe { ioctl(STDOUT_FILENO, libc::TIOCGWINSZ, &size) };

    match result {
        0 => unsafe {
            let size = size.assume_init();
            Ok((size.ws_col, size.ws_row))
        },
        _ => Err(io::Error::last_os_error()),
    }
}

fn enable_raw_mode() -> io::Result<termios> {
    let mut original = std::mem::MaybeUninit::<termios>::zeroed();
    unsafe {
        // Get original terminal attributes
        if libc::tcgetattr(STDOUT_FILENO, original.as_mut_ptr()) != 0 {
            return Err(io::Error::last_os_error());
        }

        // Make a new set of attributes with raw enabled
        let mut raw = original.assume_init();
        libc::cfmakeraw(&mut raw);

        // Set the terminal attribute to the new raw one
        if libc::tcsetattr(STDOUT_FILENO, libc::TCSANOW, &raw) != 0 {
            return Err(io::Error::last_os_error());
        }

        // Return the original one to later restore it
        Ok(original.assume_init())
    }
}

fn disable_raw_mode(original: termios) -> io::Result<()> {
    unsafe {
        if libc::tcsetattr(STDOUT_FILENO, libc::TCSANOW, &original) != 0 {
            return Err(io::Error::last_os_error());
        }
    };
    Ok(())
}
