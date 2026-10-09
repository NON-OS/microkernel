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

//! Whether the installer has caught up. It serves one request at a time, in
//! the order they came, so once it answers a probe every request sent before
//! that probe, a launch's load among them, is done. A busy
//! installer answers the probe only later, into a reply the shell no longer
//! waits for, and the kernel drops it there.

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, OP_HEALTHCHECK, PROBE_TIMEOUT_MS};
use super::frame::build;
use super::port::port;

pub fn probe() -> bool {
    let Some(port) = port() else {
        return false;
    };
    let tx = build(OP_HEALTHCHECK);
    let mut rx = [0u8; HDR_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        PROBE_TIMEOUT_MS,
    );
    rc >= HDR_LEN as i64
}
