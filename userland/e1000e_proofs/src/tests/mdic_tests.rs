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

//! MDIC command words. Vectors are built by hand from the field positions
//! Linux e1000e defines.h gives (E1000_MDIC_*: data 15:0, register 20:16,
//! PHY address 25:21, op 27:26 with 01 write and 10 read, READY 28, ERROR
//! 30), the layout the 82574 and the PCH parts share.

use crate::phy::mdic::{decode, encode};

#[test]
fn command_words_match_the_hand_built_vectors() {
    // Read PHY 1 register 2 (PHYSID1): op 10, phy 00001, reg 00010.
    assert_eq!(encode(false, 1, 2, 0xFFFF), 0x0822_0000, "a read carries no data");
    // Write PHY 2 register 0 (BMCR) with 0x1340.
    assert_eq!(encode(true, 2, 0, 0x1340), 0x0440_1340);
    // Select page 769 at PHY 1: register 0x1F, data 769 << 5.
    assert_eq!(encode(true, 1, 0x1F, 769 << 5), 0x043F_6020);
    // Read PHY 1 register 23 (CV_SMB_CTRL once page 769 is selected).
    assert_eq!(encode(false, 1, 23, 0), 0x0837_0000);
}

#[test]
fn out_of_range_fields_cannot_reach_their_neighbours() {
    // A 6-bit address or register is cut to 5 bits, never into the op.
    assert_eq!(encode(false, 0x21, 0, 0) & 0x0C00_0000, 0x0800_0000);
    assert_eq!(encode(false, 0, 0x3F, 0) & 0x03E0_0000, 0);
    for phy in 0..32 {
        for reg in 0..32 {
            let w = encode(true, phy, reg, 0xA5A5);
            assert_eq!(((w >> 21) & 0x1F, (w >> 16) & 0x1F), (phy, reg));
            assert_eq!((w & 0xFFFF, w >> 26), (0xA5A5, 0b01));
        }
    }
}

#[test]
fn finished_words_decode_to_data_or_a_named_failure() {
    let read = encode(false, 1, 2, 0);
    assert_eq!(decode(read | 0x1000_0000 | 0x0141, false, 2), Ok(0x0141));
    assert_eq!(decode(read, false, 2), Err("MDIC not ready in 100 ms"));
    let err = read | 0x1000_0000 | 0x4000_0000;
    assert_eq!(decode(err, false, 2), Err("MDIC error: the PHY did not answer"));
    // A read that comes back for register 3 is not register 2's value.
    let other = encode(false, 1, 3, 0) | 0x1000_0000;
    assert_eq!(decode(other, false, 2), Err("MDIC answered for another register"));
    // A write is only READY and not ERROR, as Linux checks it.
    let write = encode(true, 1, 0x1F, 0x6020) | 0x1000_0000;
    assert!(decode(write, true, 0).is_ok());
}
