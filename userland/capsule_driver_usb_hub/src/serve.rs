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
//! The service inbox: a health check answered, anything else refused.
//! Nothing else is asked of the walker; class drivers talk to the hosts.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, recv_ready};

use crate::wire::HDR_LEN;

const SERVICE_INBOX: u64 = 0;
const OP_HEALTHCHECK: u16 = 0x0001;
const E_INVAL: i32 = -22;

/// Wait up to `ms` for one request and answer it; false always, so a
/// caller can fold it into a search.
pub fn answer(ms: u64) -> bool {
    let mut rx = [0u8; HDR_LEN + 64];
    let mut sender = 0u32;
    let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), ms, &mut sender);
    if !recv_ready(n) || sender == 0 || (n as usize) < HDR_LEN {
        return false;
    }
    let op = u16::from_le_bytes([rx[6], rx[7]]);
    let status = if op == OP_HEALTHCHECK { 0 } else { E_INVAL };
    let mut tx = [0u8; HDR_LEN + 4];
    tx[..16].copy_from_slice(&rx[..16]);
    tx[16..20].copy_from_slice(&4u32.to_le_bytes());
    tx[20..24].copy_from_slice(&status.to_le_bytes());
    let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
    false
}
