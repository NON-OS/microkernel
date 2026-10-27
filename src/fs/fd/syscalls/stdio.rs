// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use crate::fs::fd::error::{FdError, FdResult};

pub(crate) fn write_stdout(buf: *const u8, count: usize) -> FdResult<usize> {
    if buf.is_null() {
        return Err(FdError::NullPointer);
    }

    // SAFETY: Caller guarantees buf is valid for count bytes
    unsafe {
        let slice = core::slice::from_raw_parts(buf, count);
        // A process's bytes, as MkDebug passes them: printable ones and line
        // ends, in one write rather than one per byte.
        let kept: alloc::vec::Vec<u8> = slice
            .iter()
            .copied()
            .filter(|&b| b == b'\n' || b == b' ' || b.is_ascii_graphic())
            .collect();
        // Never on serial in a build without capsule serial output.
        if cfg!(feature = "capsule-serial-debug") {
            crate::sys::serial::passthrough(&kept);
        }
    }
    Ok(count)
}

pub(crate) fn write_stderr(buf: *const u8, count: usize) -> FdResult<usize> {
    if buf.is_null() {
        return Err(FdError::NullPointer);
    }

    // SAFETY: Caller guarantees buf is valid for count bytes
    unsafe {
        let slice = core::slice::from_raw_parts(buf, count);
        for &byte in slice.iter().filter(|_| cfg!(feature = "capsule-serial-debug")) {
            crate::sys::serial::core::write_byte(byte);
        }
    }
    Ok(count)
}

pub(crate) fn read_stdin(buf: *mut u8, count: usize) -> FdResult<usize> {
    if buf.is_null() {
        return Err(FdError::NullPointer);
    }
    if count == 0 {
        return Ok(0);
    }

    // The PS/2 keyboard buffer is a legacy driver. The microkernel does
    // not own a keyboard input source on the trusted path; reading from
    // fd 0 returns "no data" (0 bytes), the canonical non-blocking
    // empty-stream answer.
    let _ = buf;
    Ok(0)
}
