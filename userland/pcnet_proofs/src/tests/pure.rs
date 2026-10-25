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

//! The arithmetic the part depends on, against the data sheet's layouts.

use crate::constants::queue::{bcnt, ENP, ERR, STP};
use crate::init::chip::part_number;
use crate::init::init_block::encode;
use crate::queue::rx_consume::frame_len;
use crate::setup::grant::fits_32;

#[test]
fn byte_counts_are_negative_with_ones_on_top() {
    assert_eq!(bcnt(1536), 0xFA00);
    assert_eq!(bcnt(60), 0xFFC4);
    assert_eq!(bcnt(1514), 0xFA16);
}

#[test]
fn a_frame_length_drops_the_fcs_and_refuses_the_rest() {
    let whole = STP | ENP;
    assert_eq!(frame_len(whole, 64), 60);
    assert_eq!(frame_len(whole, 1518), 1514);
    assert_eq!(frame_len(whole, 1519), 0, "longer than an untagged frame");
    assert_eq!(frame_len(whole, 17), 0, "a runt");
    assert_eq!(frame_len(whole | ERR, 64), 0, "errored");
    assert_eq!(frame_len(STP, 64), 0, "no end of packet");
    assert_eq!(frame_len(ENP, 64), 0, "no start of packet");
    assert_eq!(frame_len(whole, 3), 0, "shorter than its own FCS");
}

#[test]
fn only_ids_with_amds_code_name_a_part() {
    assert_eq!(part_number(0x0262_1003), Some(0x2621));
    assert_eq!(part_number(0x0262_5003), Some(0x2625));
    assert_eq!(part_number(0x0262_1000), None);
    assert_eq!(part_number(0xFFFF_FFFF), None);
}

// Am79C970A data sheet, Table 31: SSIZE32 init block.
#[test]
fn the_init_block_is_laid_out_as_the_data_sheet_says() {
    let mac = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
    let b = encode(&mac, 0x0010_0100, 0x0010_0300);
    assert_eq!(&b[0..4], &[0, 0, 0x50, 0x50]);
    assert_eq!(&b[4..10], &mac);
    assert_eq!(&b[10..20], &[0; 10]);
    assert_eq!(&b[20..24], &0x0010_0100u32.to_le_bytes());
    assert_eq!(&b[24..28], &0x0010_0300u32.to_le_bytes());
}

#[test]
fn a_grant_must_end_at_or_below_4_gib() {
    assert!(fits_32(0, 1 << 32));
    assert!(fits_32(0xFFFF_F000, 0x1000));
    assert!(!fits_32(0xFFFF_F000, 0x2000));
    assert!(!fits_32(u64::MAX, 1));
}
