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

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};
use nonos_policy_proto::{Field, HDR_LEN, POLICY_SERVICE_NAME};

use super::wire::{decode_str, request, REQ_LEN};

/// A name is short, and this runs once per field. A quarter of a second is
/// long enough for a live policy server and short enough that a dead one does
/// not stall the first command of the session.
const TIMEOUT_MS: u64 = 250;

const RX_LEN: usize = HDR_LEN + 64;

/*
 * The string policy holds for `field`, as sent. The caller sanitizes it: a
 * hostname and a user name allow different bytes.
 */
pub fn string(field: Field, out: &mut [u8]) -> Option<usize> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        POLICY_SERVICE_NAME.as_ptr(),
        POLICY_SERVICE_NAME.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || port == 0 {
        return None;
    }
    let field = field as u32;
    let mut tx = [0u8; REQ_LEN];
    request(field, &mut tx);
    let mut rx = [0u8; RX_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < HDR_LEN as i64 || n as usize > RX_LEN {
        return None;
    }
    let body = decode_str(field, &rx[..n as usize])?;
    let k = body.len().min(out.len());
    out[..k].copy_from_slice(&body[..k]);
    Some(k)
}
