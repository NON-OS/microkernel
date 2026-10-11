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

//! One SETTLE (op 10) to capsule_nox_settle, on settle_proto's wire: a drain
//! round now, so an install's voucher is on the chain before the install
//! goes on. Only the meter holds vouchers, and after
//! expiry plus GRACE anyone may close: a buyer who blocked every drain could
//! otherwise install and take the deposit back.

use alloc::vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_settle_proto::{reply_body, request, ReplyError, Settled, EAGAIN, EIO};
use nonos_settle_proto::{HDR_LEN, OP_SETTLE, SERVICE, SETTLED_MAX, STATUS_LEN};

/// A round reads the chain and broadcasts over Nym or Anyone.
const TIMEOUT_MS: u64 = 300_000;

/// The settle() a round broadcast on `chain_id`, and what it carries.
pub fn settle_now(chain_id: u64) -> Result<Settled, i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(EAGAIN);
    }
    let id = mk_time_millis() as u32;
    let tx = request(OP_SETTLE, id, &chain_id.to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + 33 + 48 * SETTLED_MAX];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return Err(EAGAIN);
    }
    match reply_body(rx.get(..n as usize).unwrap_or(&[]), OP_SETTLE, id) {
        Ok(b) => Settled::parse(b).ok_or(EIO),
        Err(ReplyError::Status(s)) => Err(s),
        Err(_) => Err(EIO),
    }
}
