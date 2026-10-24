// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! This driver's leg of the leave phase (block_device::leave): a flush, as
//! the host holds the cache.

use alloc::vec;

use super::super::protocol::{decode_response, encode_request, OP_FLUSH};
use super::super::state;
use super::seq::next_request_id;
use super::transport::{REPLY_INBOX, SENDER_NAME, TRANSPORT_LOCK};
use crate::hardware::block_device::leave::{Leg, Wire};

const BUDGET_MS: u64 = 5_000;

pub(crate) fn leave_leg() -> Option<Leg> {
    let id = next_request_id();
    Leg::new(
        "driver.virtio_blk0",
        BUDGET_MS,
        wire(),
        vec![(id, encode_request(OP_FLUSH, 0, id, &[]))],
    )
}

/// The wire as the leave phase holds it: the round trip's own lock, so its
/// request is never sent behind another caller's or its reply taken by one.
fn wire() -> Wire {
    Wire {
        lock: &TRANSPORT_LOCK,
        sender: SENDER_NAME,
        reply_inbox: REPLY_INBOX,
        state: state::shared_state(),
        decode: decode_response,
    }
}
