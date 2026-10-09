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

//! How long a sent NTB is made, as the end of Linux cdc_ncm_fill_tx_frame
//! decides. The device learns where a block ends from a short packet; the
//! controller cannot send a zero-length one here, so a block never ends on
//! a packet boundary unless it is the device's own dwNtbOutMaxSize, the
//! one length cdc_ncm_update_rxtx_max leaves without its extra byte.

use super::limits::ends_on_packet;
use super::shape::TxShape;

pub fn padded(len: usize, s: &TxShape) -> usize {
    if !s.zlp && len > s.min_pkt {
        // A device with a DMA engine takes a full-size block faster than
        // one cut short; tx_max itself does not end on a packet boundary.
        s.max
    } else if len < s.max && ends_on_packet(len, s.mps) {
        len + 1
    } else {
        len
    }
}
