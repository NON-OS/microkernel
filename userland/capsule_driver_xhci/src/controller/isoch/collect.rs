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

//! Collecting an isoch IN batch's events, one per TD, so each packet's
//! length is known; TDs complete in ring order, so the last TD's event ends
//! it. A same-endpoint event naming no TD of the batch (an overrun, or a
//! missed-service report) is dropped, and a TD left without an event counts
//! as missed. Other endpoints' events are parked.

use nonos_libc::Deadline;

use super::inbound::{Got, InBatch, ISOCH_TIMEOUT_MS};
use crate::constants::{CC_MISSED_SERVICE, TRB_TYPE_TRANSFER_EVENT};
use crate::controller::park::{park_step, SPIN_BUDGET};
use crate::error::{XhciError, XhciResult};
use crate::protocol::ISOCH_PACKETS_MAX;
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};

pub(super) fn collect(
    intr_base: u64,
    (slot, dci): (u8, u8),
    b: &InBatch,
    evt: &mut EventRing,
) -> XhciResult<Got> {
    let mut got: Got = [(CC_MISSED_SERVICE, b.td_len); ISOCH_PACKETS_MAX];
    let td = |i: usize| IssuedTransfer { phys: b.tds[i], slot, dci };
    let mut last_seen = false;
    for (i, g) in got[..b.n].iter_mut().enumerate() {
        if let Some(e) = evt.take_parked(td(i)) {
            *g = (e.completion_code(), e.d2 & 0x00FF_FFFF);
            last_seen |= i + 1 == b.n;
        }
    }
    let (deadline, mut spins) = (Deadline::after_ms(ISOCH_TIMEOUT_MS), 0u32);
    while !last_seen {
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::TransferCompletionTimeout);
        }
        if !evt.has_event() {
            park_step(spins);
            continue;
        }
        let e = evt.current_trb();
        evt.advance();
        erdp_program(intr_base, evt.current_dequeue_phys(), true, 0);
        if e.get_type() != TRB_TYPE_TRANSFER_EVENT {
            continue;
        }
        if e.slot_id() != slot || e.endpoint_id() != dci {
            evt.park(e);
        } else if let Some(i) = (0..b.n).find(|&i| td(i).completed_by(&e)) {
            got[i] = (e.completion_code(), e.d2 & 0x00FF_FFFF);
            last_seen = i + 1 == b.n;
        }
    }
    Ok(got)
}
