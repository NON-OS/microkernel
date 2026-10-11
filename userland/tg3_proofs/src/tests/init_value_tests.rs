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

//! The words tg3_reset_hw works out for the 57765 class, worked by hand
//! from tg3.c and tg3.h and held against the driver's: GRC mode, the DMA
//! control word, the replenish thresholds, the standard ring's control
//! word and which ring control blocks are disabled.

use crate::constants::chip::CHIPREV_ID_57765_A0;
use crate::init::{bufmgr, grc, rcbs, rings};

use super::build::REV_57766;

#[test]
fn grc_mode_is_the_swaps_with_host_send_bds() {
    // WSWAP_DATA | BSWAP_DATA | WSWAP_NONFRM_DATA | HOST_SENDBDS |
    // NO_TX_PHDR_CSUM.
    assert_eq!(grc::grc_mode(), 0x0012_0034);
}

#[test]
fn dma_control_sets_cache_alignment_off_and_a0_drops_the_read_size() {
    assert_eq!(grc::dma_rw_ctrl(0, REV_57766), 0x1);
    assert_eq!(grc::dma_rw_ctrl(0x0000_0380, REV_57766), 0x381);
    assert_eq!(grc::dma_rw_ctrl(0x0000_0380, CHIPREV_ID_57765_A0), 0x1);
    assert_eq!(grc::dma_rw_ctrl(0xFFFF_FFFE, CHIPREV_ID_57765_A0), 0xFFFF_FC7F);
    assert_eq!(grc::dma_rw_ctrl(0xFFFF_FFFF, REV_57766), 0xFFFF_FFFF);
}

#[test]
fn replenish_thresholds_are_the_lesser_half() {
    // min(128 / 2, max(posted / 8, 1)).
    assert_eq!(bufmgr::std_thresh(64), 8);
    assert_eq!(bufmgr::std_thresh(200), 25);
    assert_eq!(bufmgr::std_thresh(1024), 64);
    assert_eq!(bufmgr::std_thresh(0), 1);
    // min(64 / 2, max(100 / 8, 1)).
    assert_eq!(bufmgr::jumbo_thresh(), 12);
}

#[test]
fn the_standard_ring_declares_its_size_and_buffer() {
    // 512 << 16 | 1536 << 2.
    assert_eq!(rings::std_maxlen_flags(), 0x0200_1800);
}

#[test]
fn every_ring_past_the_first_is_disabled() {
    // One more send ring at 0x100 + 16, three more return rings past 0x200.
    let at: Vec<u32> = rcbs::disabled().collect();
    assert_eq!(at, [0x118, 0x218, 0x228, 0x238]);
}
