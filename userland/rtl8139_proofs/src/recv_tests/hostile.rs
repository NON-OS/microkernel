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

//! Headers no good frame carries: each restarts receive from the top of the
//! ring, and nothing is copied for it.

use super::ring_with;
use crate::constants::regs::RX_STATUS_OK;
use crate::part::{driver_over, part};
use crate::rx::recv_one;

#[test]
fn a_header_no_good_frame_carries_restarts_receive_and_copies_nothing() {
    let hostile: [(u16, u16); 5] = [
        (0, 68),              // status without receive-OK
        (RX_STATUS_OK, 0),    // no length at all
        (RX_STATUS_OK, 7),    // shorter than any header
        (RX_STATUS_OK, 1797), // past the longest frame the part hands up
        (RX_STATUS_OK, 0xFFFF),
    ];
    for (status, raw_len) in hostile {
        let _part = part(false);
        let ring = ring_with(status, raw_len, 0);
        let mut d = driver_over(ring.as_ptr() as u64);
        d.rx_offset = 0;
        let mut out = [0xEEu8; 2048];
        assert_eq!(
            recv_one(&mut d, &mut out),
            Err("rtl8139 rx ring restarted"),
            "status {status:#x} length {raw_len} was trusted"
        );
        assert!(out.iter().all(|b| *b == 0xEE), "length {raw_len} copied something");
        assert_eq!(d.rx_offset, 0, "receive starts over from the top of the ring");
    }
}
