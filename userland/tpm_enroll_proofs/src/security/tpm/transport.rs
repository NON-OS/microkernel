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

//! The kernel's `transact`, over a socket instead of the CRB window.
//!
//! One connection per command. swtpm serves one client at a time and keeps
//! its state across connections, so tpm2-tools can take the port between two
//! of the kernel's commands, as the registrar's reference, without the
//! kernel's transient objects or sessions going anywhere.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicU16, Ordering};

use super::error::TpmError;

/// The command port of the swtpm the running test started.
pub static PORT: AtomicU16 = AtomicU16::new(0);

/// As in the kernel, a response longer than `out` is refused, not truncated.
///
/// # Safety
/// None needed here; the signature is the kernel's.
pub unsafe fn transact(cmd: &[u8], out: &mut [u8]) -> Result<usize, TpmError> {
    let port = PORT.load(Ordering::SeqCst);
    let mut s = TcpStream::connect(("127.0.0.1", port)).map_err(|_| TpmError::NotPresent)?;
    s.write_all(cmd).map_err(|_| TpmError::Timeout)?;
    let mut head = [0u8; 10];
    s.read_exact(&mut head).map_err(|_| TpmError::Timeout)?;
    let size = u32::from_be_bytes([head[2], head[3], head[4], head[5]]) as usize;
    if size < head.len() || size > out.len() {
        return Err(TpmError::InvalidResponse);
    }
    out[..10].copy_from_slice(&head);
    s.read_exact(&mut out[10..size]).map_err(|_| TpmError::Timeout)?;
    Ok(size)
}
