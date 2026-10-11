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

//! A flow's ask as settle_proto's request, and settle's answer to it.

use alloc::vec::Vec;

use nonos_nox_meter::flow::Ask;
use nonos_settle_proto::{parse_read_reply, Gas, Read, OP_BROADCAST, OP_GAS, OP_READ};

/// Settlement's GRACE() (settle_proto view 16).
const VIEW_GRACE: u16 = 16;

pub(super) fn request(ask: &Ask, chain_id: u64, meter: &[u8; 20]) -> Option<(u16, Vec<u8>)> {
    Some(match ask {
        Ask::Grace => (OP_READ, Read { chain_id, view: VIEW_GRACE, args: Vec::new() }.encode()?),
        Ask::Gas => (OP_GAS, Gas::request(chain_id, meter).to_vec()),
        Ask::Broadcast(raw) => (OP_BROADCAST, [&chain_id.to_le_bytes()[..], raw].concat()),
    })
}

/// GRACE as seconds: one uint word that fits u64.
pub(super) fn grace(body: &[u8]) -> Option<u64> {
    let w = *parse_read_reply(body)?.first()?;
    w[..24].iter().all(|b| *b == 0).then(|| u64::from_be_bytes(w[24..].try_into().unwrap()))
}

pub(super) fn hash(body: &[u8]) -> Option<[u8; 32]> {
    body.try_into().ok()
}
