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

//! A bulk transfer's Transfer Event residual (bits 23:0 of its third dword)
//! counts the requested bytes that did not move, and the controller writes
//! anything up to 0xFF_FFFF there: what moved is the request less the
//! residual clamped to it.

use super::events::{transfer, CC_SHORT_PACKET, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, SLOT};
use crate::controller::bulk_transfer;
use crate::protocol::BULK_MAX;

#[test]
fn a_bulk_transfer_moves_the_request_less_the_residual_clamped_to_it() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    for len in [1u32, 512, BULK_MAX as u32] {
        for r in [0, 1, len - 1, len, len + 1, 0xFFFF, 0x00FF_FFFF] {
            for code in [CC_SUCCESS, CC_SHORT_PACKET] {
                let issued = pipes.in_ring.enqueue_phys();
                let mut event = transfer(issued, code, 0, SLOT, BULK_IN_DCI);
                event.d2 = (code as u32) << 24 | r;
                fx.hc.post(event);
                let moved =
                    bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, len);
                assert_eq!(moved, Ok(len - r.min(len)), "length {len}, residual {r:#x}");
            }
        }
    }
}
