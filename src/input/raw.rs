use std::{
    ffi::c_void,
    io::{self, Write},
};

use libc::{POLLERR, POLLHUP, POLLIN, STDIN_FILENO};

pub fn input_available() -> io::Result<bool> {
    let mut poll = libc::pollfd {
        fd: STDIN_FILENO,
        events: POLLIN,
        revents: 0,
    };

    let result = unsafe { libc::poll(&mut poll, 1, 0) };

    if result < 0 {
        // Error polling
        return Err(io::Error::last_os_error());
    }
    if result == 0 {
        // Nothing to read
        return Ok(false);
    }

    if poll.revents & POLLIN != 0 {
        // Yippee something to read
        return Ok(true);
    }

    if poll.revents & POLLERR != 0 {
        // Polling error occured
        return Err(io::Error::new(io::ErrorKind::Other, "stdin poll failed"));
    }

    if poll.revents & POLLHUP != 0 {
        // stdin hung up :(
        return Err(io::Error::new(io::ErrorKind::Other, "stdin closed"));
    }

    Ok(false)
}

pub fn read_bytes(buf: &mut [u8]) -> io::Result<usize> {
    let num = unsafe { libc::read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, buf.len()) };

    if num < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(num as usize)
}

pub fn read_byte() -> io::Result<Option<u8>> {
    let mut buf = 0u8; // single byte buffer 
    let num = unsafe { libc::read(STDIN_FILENO, &mut buf as *mut u8 as *mut c_void, 1) };

    if num < 0 {
        // Error reading
        return Err(io::Error::last_os_error());
    }

    if num == 0 {
        // End of input
        return Ok(None);
    }

    return Ok(Some(buf));
}
