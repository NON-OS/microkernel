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

//! The genuine event a wait is owed, and the random dwords both it and the
//! hostile events draw from.

use super::rng::Rng;
use crate::event_ring::events::{transfer, CC_SHORT_PACKET, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use crate::event_ring::fixture::SLOT;
use crate::trb::Trb;

pub(super) fn random_trb(rng: &mut Rng) -> Trb {
    let r = rng.next();
    Trb { d0: r as u32, d1: (r >> 32) as u32, d2: rng.next() as u32, d3: rng.next() as u32 }
}

/// A completion code and residual for a `request`-byte TRB, from the edges.
pub(super) fn third_dword(rng: &mut Rng, request: u32) -> u32 {
    let code = rng.pick(&[CC_SUCCESS, CC_SHORT_PACKET, CC_STALL, CC_TRB_ERROR, 0, 0xFF]) as u32;
    let residual = rng.pick(&[0, 1, request.saturating_sub(1), request, request + 1, 0x00FF_FFFF]);
    let near = rng.below(request as u64 + 2) as u32;
    let any = rng.next() as u32;
    rng.pick(&[u32::MAX, code << 24 | residual, code << 24 | near, any])
}

/// The event that names `issued` on `dci`, as a working controller would
/// write it, with a completion code and residual from the edges.
pub(super) fn genuine(rng: &mut Rng, issued: u64, dci: u8, request: u32) -> Trb {
    let mut t = transfer(issued, CC_SUCCESS, 0, SLOT, dci);
    if rng.coin() {
        t.d2 = third_dword(rng, request);
    }
    t
}
