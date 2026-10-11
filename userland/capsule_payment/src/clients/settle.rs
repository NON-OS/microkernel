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

//! OP_CHANNEL_STATE from capsule_nox_settle. Best effort: a settle that is
//! down, slow, or whose two RPC hosts disagree leaves the meter on its cached
//! state and its local floor, which is what an airgapped image runs on.

use alloc::vec;

use nonos_libc::{mk_ipc_call_timeout, mk_time_millis};
use nonos_nox_meter::state::ChainView;
use nonos_settle_proto::{chain_hash, reply_body, request, ChannelState};
use nonos_settle_proto::{CHANNEL_STATE_LEN, HDR_LEN, OP_CHANNEL_STATE, SERVICE, STATUS_LEN};

use super::lookup::port_of;

const TIMEOUT_MS: u64 = 1500;

pub(crate) fn channel_state(chain_id: u64, channel_id: &[u8; 32]) -> Option<ChainView> {
    let port = port_of(SERVICE)?;
    let id = mk_time_millis() as u32;
    let tx = request(OP_CHANNEL_STATE, id, &chain_hash(chain_id, channel_id));
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + CHANNEL_STATE_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n <= 0 {
        return None;
    }
    let body = reply_body(&rx[..n as usize], OP_CHANNEL_STATE, id).ok()?;
    let s = ChannelState::parse(body)?;
    Some(ChainView { deposit: s.deposited, settled: s.settled, expiry: s.expiry, closed: s.closed })
}
