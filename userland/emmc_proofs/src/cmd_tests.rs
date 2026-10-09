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

//! The Command and Transfer Mode words for every command the driver sends,
//! against values written out from the specifications.

use crate::emmc::mmc::cmds::*;
use crate::emmc::mmc::io::{plan, Plan};
use crate::emmc::sdhci::cmd::{command_word, transfer_mode, Resp};

#[test]
fn every_command_carries_its_response_type_checks_and_data_flag() {
    // (index, response, data, abort, word): bits 13:8 index, 7:6 type,
    // 5 data present, 4 index check, 3 CRC check, 1:0 response type.
    let table: &[(u8, Resp, bool, bool, u16)] = &[
        (GO_IDLE_STATE, Resp::None, false, false, 0x0000),
        (SEND_OP_COND, Resp::R3, false, false, 0x0102),
        (ALL_SEND_CID, Resp::R2, false, false, 0x0209),
        (SET_RELATIVE_ADDR, Resp::R1, false, false, 0x031a),
        (SWITCH, Resp::R1b, false, false, 0x061b),
        (SELECT_CARD, Resp::R1, false, false, 0x071a),
        (SEND_EXT_CSD, Resp::R1, true, false, 0x083a),
        (SEND_CSD, Resp::R2, false, false, 0x0909),
        (STOP_TRANSMISSION, Resp::R1b, false, true, 0x0cdb),
        (SEND_STATUS, Resp::R1, false, false, 0x0d1a),
        (SET_BLOCKLEN, Resp::R1, false, false, 0x101a),
        (READ_SINGLE_BLOCK, Resp::R1, true, false, 0x113a),
        (READ_MULTIPLE_BLOCK, Resp::R1, true, false, 0x123a),
        (SET_BLOCK_COUNT, Resp::R1, false, false, 0x171a),
        (WRITE_BLOCK, Resp::R1, true, false, 0x183a),
        (WRITE_MULTIPLE_BLOCK, Resp::R1, true, false, 0x193a),
    ];
    for &(i, r, d, a, w) in table {
        assert_eq!(command_word(i, r, d, a), w, "CMD{i}");
    }
}

#[test]
fn only_r1b_holds_the_data_line_and_only_r2_is_long() {
    for r in [Resp::None, Resp::R1, Resp::R2, Resp::R3] {
        assert!(!r.busy());
    }
    assert!(Resp::R1b.busy());
    assert!(Resp::R2.long());
    assert!(!Resp::R1.long() && !Resp::R3.long());
}

#[test]
fn transfer_mode_bits_for_each_kind_of_transfer() {
    // bit 0 DMA, 1 block count, 3:2 auto CMD (01 = CMD12), 4 read, 5 multi.
    assert_eq!(transfer_mode(true, false, false), 0x13);
    assert_eq!(transfer_mode(false, false, false), 0x03);
    assert_eq!(transfer_mode(true, true, false), 0x33);
    assert_eq!(transfer_mode(false, true, false), 0x23);
    assert_eq!(transfer_mode(true, true, true), 0x37);
    assert_eq!(transfer_mode(false, true, true), 0x27);
    // Auto CMD12 is never asked for a single block.
    assert_eq!(transfer_mode(true, false, true), 0x13);
    // Auto CMD23 (10b) is never used: the driver sends CMD23 itself.
    for r in [false, true] {
        for m in [false, true] {
            for a in [false, true] {
                assert_eq!(transfer_mode(r, m, a) & 0x0c & 0x08, 0);
            }
        }
    }
}

#[test]
fn the_plan_for_each_size_and_card() {
    assert_eq!(plan(1, false, true), Plan { cmd23: false, index: 17, multi: false, auto12: false });
    assert_eq!(plan(1, true, true), Plan { cmd23: false, index: 24, multi: false, auto12: false });
    assert_eq!(plan(2, false, true), Plan { cmd23: true, index: 18, multi: true, auto12: false });
    assert_eq!(plan(64, true, true), Plan { cmd23: true, index: 25, multi: true, auto12: false });
    assert_eq!(plan(2, false, false), Plan { cmd23: false, index: 18, multi: true, auto12: true });
    assert_eq!(plan(64, true, false), Plan { cmd23: false, index: 25, multi: true, auto12: true });
    assert_eq!(
        plan(1, false, false),
        Plan { cmd23: false, index: 17, multi: false, auto12: false }
    );
}

#[test]
fn arguments() {
    assert_eq!(rca_arg(1), 0x0001_0000);
    assert_eq!(switch_arg(185, 1), 0x03b9_0100);
    assert_eq!(switch_arg(183, 2), 0x03b7_0200);
    assert_eq!(switch_arg(179, 0x48), 0x03b3_4800);
    assert_eq!(switch_arg(32, 1), 0x0320_0100);
    assert_eq!(block_count_arg(64), 64);
    assert_eq!(data_arg(7, true), Some(7));
    assert_eq!(data_arg(u32::MAX as u64, true), Some(u32::MAX));
    assert_eq!(data_arg(1 << 32, true), None);
    assert_eq!(data_arg(7, false), Some(7 * 512));
    assert_eq!(data_arg((1 << 23) - 1, false), Some(((1 << 23) - 1) * 512));
    assert_eq!(data_arg(1 << 23, false), None);
    assert_eq!(data_arg(u64::MAX, false), None);
}
