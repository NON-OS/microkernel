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
use crate::protocol::{
    CONTROLLER_STATUS_PAYLOAD_LEN, MAX_POLL_EVENTS, MOUSE_EVENT_WIRE_LEN,
    MOUSE_POLL_PAYLOAD_PREFIX_LEN, RESP_HDR_LEN, STATE_PAYLOAD_LEN, STATUS_LEN,
};

pub(super) const POLL_IDLE_MS: u64 = 1;
pub(super) const IRQ_WAIT_MS: u64 = 100;

/// The longest reply any op sends: a full mouse batch, the counters, or
/// the controller status.
pub(super) const TX_LEN: usize = max(
    RESP_HDR_LEN + MOUSE_POLL_PAYLOAD_PREFIX_LEN + MAX_POLL_EVENTS * MOUSE_EVENT_WIRE_LEN,
    max(
        RESP_HDR_LEN + STATUS_LEN + STATE_PAYLOAD_LEN,
        RESP_HDR_LEN + STATUS_LEN + CONTROLLER_STATUS_PAYLOAD_LEN,
    ),
);

const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}
