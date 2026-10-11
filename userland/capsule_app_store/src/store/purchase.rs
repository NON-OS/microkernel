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

//! 19/0 POST (plan/21, plan/27): the selected listing's names, and only its
//! names, into the meter's mailbox for the wallet to take.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::{reply_body, request, Names, EAGAIN, EINVAL, HDR_LEN};
use nonos_payment_proto::{OP_PURCHASE, SERVICE, STATUS_LEN};

use super::asked::Asked;
use super::purchase_said::handed;
use super::state::State;

/// The meter answers from memory.
const TIMEOUT_MS: u64 = 2_000;

pub fn post(state: &State) -> Asked {
    let Some(l) = state.current() else { return Asked::NotInstallable };
    let names = Names { chain_id: SEPOLIA, listing: l.id.clone(), release: Vec::new() };
    handed(names.post().ok_or(EINVAL).and_then(|b| call(&b)))
}

fn call(body: &[u8]) -> Result<(), i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(EAGAIN);
    }
    let id = mk_time_millis() as u32;
    let tx = request(OP_PURCHASE, id, body);
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return Err(EAGAIN);
    }
    match reply_body(&rx[..n as usize], OP_PURCHASE, id) {
        Ok(_) => Ok(()),
        Err(nonos_payment_proto::ReplyError::Status(s)) => Err(s),
        Err(_) => Err(EAGAIN),
    }
}
