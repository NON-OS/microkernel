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

//! ADMA2 descriptor sizes, tables refused for their length, alignment, reach
//! or room, and the descriptor width a region's address calls for.

use crate::emmc::sdhci::adma::*;

#[test]
fn descriptor_sizes() {
    assert_eq!(Width::A32.desc_len(), 8);
    assert_eq!(Width::A64.desc_len(), 12);
    assert_eq!((Width::A32.align(), Width::A64.align()), (4, 8));
}

#[test]
fn bad_lengths_alignment_and_reach_build_nothing() {
    let mut t = [0u8; TABLE_BYTES];
    assert_eq!(build(&mut t, Width::A32, 0x1000, 0), Err(AdmaError::Length));
    assert_eq!(build(&mut t, Width::A32, 0x1000, 510), Err(AdmaError::Length));
    assert_eq!(build(&mut t, Width::A32, 0x1002, 512), Err(AdmaError::Align));
    assert_eq!(build(&mut t, Width::A64, 0x1004, 512), Err(AdmaError::Align));
    assert_eq!(build(&mut t, Width::A32, 0xffff_ff00, 512), Err(AdmaError::Reach));
    assert_eq!(build(&mut t, Width::A32, 0x1_0000_0000, 512), Err(AdmaError::Reach));
    assert_eq!(build(&mut t, Width::A64, u64::MAX - 7, 512), Err(AdmaError::Reach));
    assert_eq!(build(&mut t[..8], Width::A32, 0x1000, 65536), Err(AdmaError::Room));
    assert!(t.iter().all(|&b| b == 0));
    // The last 32-bit byte is reachable.
    assert_eq!(build(&mut t, Width::A32, 0xffff_fe00, 512), Ok(1));
    assert_eq!(build(&mut t, Width::A64, 0x1_0000_0000, 512), Ok(1));
}

#[test]
fn width_is_32_bit_below_4_gib_else_64_bit_if_the_host_has_it() {
    assert_eq!(width_for(0x1000, 4096, 0x2000, 32768, false), Some(Width::A32));
    assert_eq!(width_for(0x1000, 4096, 0x2000, 32768, true), Some(Width::A32));
    assert_eq!(width_for(0x1_0000_0000, 4096, 0x2000, 512, true), Some(Width::A64));
    assert_eq!(width_for(0x1000, 4096, 0xffff_f000, 8192, true), Some(Width::A64));
    assert_eq!(width_for(0x1000, 4096, 0xffff_f000, 8192, false), None);
    assert_eq!(width_for(0x1000, 4096, u64::MAX, 1, false), None);
}
