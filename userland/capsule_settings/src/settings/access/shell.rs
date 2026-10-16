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

//! The shell's two accessibility ops, called with Settings' own pid on them;
//! the shell checks that pid and passes the body to the compositor.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const SHELL: &[u8] = b"desktop_shell";
const NDSH_MAGIC: u32 = 0x4E44_5348;
const NDSH_VERSION: u16 = 1;
const HDR_LEN: usize = 20;
pub const OP_ACCESSIBILITY: u16 = 0x0009;
pub const OP_ACCESSIBILITY_GET: u16 = 0x000A;
const TIMEOUT_MS: u64 = 1000;
/// No shell, no answer, or one too short to read.
pub const E_IO: i32 = -5;

/// Call `op` with `body`; the answer's payload is copied into `out`.
pub fn call(op: u16, body: &[u8], out: &mut [u8]) -> Result<(), i32> {
    let mut port: u32 = 0;
    let rc = mk_service_lookup(SHELL.as_ptr(), SHELL.len(), &mut port as *mut u32, ptr::null_mut());
    if rc != 0 || port == 0 {
        return Err(E_IO);
    }
    let mut tx = [0u8; HDR_LEN + 20];
    tx[0..4].copy_from_slice(&NDSH_MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&NDSH_VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    let n = HDR_LEN + body.len().min(20);
    tx[HDR_LEN..n].copy_from_slice(&body[..n - HDR_LEN]);
    let mut rx = [0u8; HDR_LEN + 4 + 16];
    let want = HDR_LEN + 4 + out.len().min(16);
    let got = mk_ipc_call_timeout(port as u64, tx.as_ptr(), n, rx.as_mut_ptr(), want, TIMEOUT_MS);
    if got < (HDR_LEN + 4) as i64 || rx[0..4] != NDSH_MAGIC.to_le_bytes() {
        return Err(E_IO);
    }
    match i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) {
        0 if (got as usize) < want => Err(E_IO),
        0 => {
            let len = out.len().min(16);
            out[..len].copy_from_slice(&rx[HDR_LEN + 4..HDR_LEN + 4 + len]);
            Ok(())
        }
        e => Err(e),
    }
}
