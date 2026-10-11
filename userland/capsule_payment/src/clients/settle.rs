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
//! down or slow leaves the meter on its cached state and its local floor,
//! which is what an airgapped image runs on.

use nonos_libc::mk_ipc_call_timeout;
use nonos_nox_meter::state::ChainView;
use nonos_settle_proto::{channel_state_request, parse_channel_state, CHANNEL_STATE_LEN};
use nonos_settle_proto::{OP_CHANNEL_STATE, SERVICE};

use super::lookup::port_of;

const TIMEOUT_MS: u64 = 1500;

pub(crate) fn channel_state(chain_id: u64, channel_id: &[u8; 32]) -> Option<ChainView> {
    let port = port_of(SERVICE)?;
    let mut tx = [0u8; 8 + 40];
    tx[4..6].copy_from_slice(&OP_CHANNEL_STATE.to_le_bytes());
    tx[8..].copy_from_slice(&channel_state_request(chain_id, channel_id));
    let mut rx = [0u8; 8 + CHANNEL_STATE_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n != rx.len() as i64 || rx[4..8] != [0; 4] {
        return None;
    }
    let s = parse_channel_state(&rx[8..])?;
    Some(ChainView { deposit: s.deposited, settled: s.settled, expiry: s.expiry, closed: s.closed })
}
