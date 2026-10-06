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

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;

use super::fault::{fault, Fault};
use super::pace::POLL_MS;

/// Largest single answer worth taking from the proxy.
const REPLY_MAX: usize = 36 * 1024;

/// Hand one frame to the proxy at `port` and return what it says back,
/// waiting at most `POLL_MS`.
pub fn exchange(port: u32, frame: &[u8]) -> Result<Vec<u8>, Fault> {
    let mut rx = vec![0u8; REPLY_MAX];
    let n = mk_ipc_call_timeout(
        port as u64,
        frame.as_ptr(),
        frame.len(),
        rx.as_mut_ptr(),
        rx.len(),
        POLL_MS,
    );
    if n < 0 {
        return Err(fault(n));
    }
    rx.truncate(n as usize);
    Ok(rx)
}
