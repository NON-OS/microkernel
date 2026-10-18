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

//! The hostile events the fuzzing controller writes; the genuine one a
//! wait is owed is `genuine`.

use super::genuine::{random_trb, third_dword};
use super::rng::Rng;
use crate::constants::{TRB_TYPE_CMD_COMPLETION_EVENT, TRB_TYPE_TRANSFER_EVENT};
use crate::event_ring::events::PORT_STATUS_CHANGE;
use crate::event_ring::fixture::{EP0_DCI, SLOT, TRB};
use crate::trb::Trb;

/// An event with every dword random, its steering fields half the time from
/// the edges named above, aimed at the TRB at `issued` on endpoint `dci`.
pub(super) fn hostile(rng: &mut Rng, issued: u64, dci: u8, request: u32) -> Trb {
    let mut t = random_trb(rng);
    if rng.coin() {
        let ty = rng.pick(&[
            TRB_TYPE_TRANSFER_EVENT,
            TRB_TYPE_CMD_COMPLETION_EVENT,
            PORT_STATUS_CHANGE,
            0,
            63,
        ]);
        t.set_type(ty);
    }
    if rng.coin() {
        let low = rng.below(16);
        let p = rng.pick(&[
            issued,
            issued | low,
            issued + TRB,
            issued.wrapping_sub(TRB),
            issued ^ (1 << 40),
            0,
            low,
            u64::MAX,
        ]);
        t.set_pointer(p);
    }
    if rng.coin() {
        let slot = rng.pick(&[SLOT, 0, 255, SLOT + 1]) as u32;
        t.d3 = (t.d3 & 0x00FF_FFFF) | slot << 24;
    }
    if rng.coin() {
        let ep = rng.pick(&[dci, 0, 31, EP0_DCI]) as u32;
        t.d3 = (t.d3 & !(0x1F << 16)) | ep << 16;
    }
    if rng.coin() {
        t.d2 = third_dword(rng, request);
    }
    t
}
