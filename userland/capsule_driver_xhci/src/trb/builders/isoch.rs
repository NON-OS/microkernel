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

//! An Isoch TRB (xHCI 1.2 section 6.4.1.3), one per TD: each packet of an
//! isochronous stream is a TD of its own, scheduled as soon as possible
//! (SIA) rather than at a frame the driver would have to track.

use crate::constants::{TRB_SIA, TRB_TYPE_ISOCH};
use crate::trb::Trb;

/// `tbc` and `tlbpc` are the TD's Transfer Burst Count and Last Burst
/// Packet Count (`controller::isoch::burst_fields`).
pub fn isoch(buffer_phys: u64, length: u32, tbc: u8, tlbpc: u8, ioc: bool, cycle: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_pointer(buffer_phys);
    trb.set_transfer_length(length);
    trb.set_type(TRB_TYPE_ISOCH);
    trb.d3 |= TRB_SIA | ((tbc as u32 & 0x3) << 7) | ((tlbpc as u32 & 0xF) << 16);
    if ioc {
        trb.set_ioc(true);
    }
    trb.set_cycle(cycle);
    trb
}
