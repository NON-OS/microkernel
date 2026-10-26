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

//! Whether the installer has finished a launch's load, and how it ended. It
//! serves one request at a time, in the order they came, so once it answers
//! a probe the load sent before it is over, and the answer is the status it
//! kept for that load (its server/outcomes.rs): a refusal reaches the shell
//! with its reason though the load's own reply came too late. A busy
//! installer answers only later, into a reply the shell no longer waits
//! for, and the kernel drops it there.

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, OP_LOAD_OUTCOME, PROBE_TIMEOUT_MS};
use super::load_wire::request;
use super::port::port;

/// How this shell's last load of `name` ended; None while the installer is
/// still too busy to say.
pub fn probe(name: &[u8]) -> Option<i32> {
    let port = port()?;
    let tx = request(OP_LOAD_OUTCOME, name);
    let mut rx = [0u8; HDR_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        PROBE_TIMEOUT_MS,
    );
    (rc >= HDR_LEN as i64).then(|| i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]))
}
