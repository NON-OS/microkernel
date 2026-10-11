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

//! A short frame reaches the part at the 17 bytes its transmit DMA takes
//! (the the review): the frame, then zeros, and the descriptor
//! names the padded length. The stack's own minimum stays 14 bytes.

use crate::constants::queue::TX_BUFFER_LEN;
use crate::constants::MIN_ETHERNET_FRAME;
use crate::queue::tx_pad::{fill, MIN_WIRE_LEN};

#[test]
fn a_bare_header_is_padded_to_seventeen_bytes() {
    for len in MIN_ETHERNET_FRAME..MIN_WIRE_LEN {
        let mut slot = vec![0xEEu8; TX_BUFFER_LEN];
        let frame: Vec<u8> = (1..=len as u8).collect();
        assert_eq!(fill(&mut slot, &frame), Some(MIN_WIRE_LEN), "{len}");
        assert_eq!(&slot[..len], &frame[..]);
        assert!(slot[len..MIN_WIRE_LEN].iter().all(|b| *b == 0), "{len}: not zeroed");
        assert_eq!(slot[MIN_WIRE_LEN], 0xEE, "past the wire length untouched");
    }
}

#[test]
fn longer_frames_keep_their_length_and_an_oversized_one_is_refused() {
    let mut slot = vec![0u8; TX_BUFFER_LEN];
    assert_eq!(fill(&mut slot, &[7u8; 60]), Some(60));
    assert_eq!(fill(&mut slot, &[7u8; 1514]), Some(1514));
    assert_eq!(fill(&mut slot[..16], &[7u8; 14]), None);
}
