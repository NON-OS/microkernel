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

//! The SD commands, OCR and card status, against the SD Physical Layer
//! Specification (4.7.4, 4.9, 5.1) and Linux's choices for this host.

use crate::regs::sd::*;
use crate::sd::ocr::IF_COND_ARG;
use crate::sd::*;

#[test]
fn the_commands_carry_their_index_argument_and_response() {
    assert_eq!(Command::SEND_IF_COND.arg, 0x1AA);
    assert_eq!(IF_COND_ARG, 0x1AA);
    assert_eq!(Command::app_cmd(0xB368).arg, 0xB368_0000);
    // 3.2 to 3.4 V, with HCS only when CMD8 was answered.
    assert_eq!(Command::sd_send_op_cond(true).arg, 0x4030_0000);
    assert_eq!(Command::sd_send_op_cond(false).arg, 0x0030_0000);
    assert_eq!(Command::sd_send_op_cond(true).rsp, Rsp::R3);
    assert_eq!(Command::select_card(1).rsp, Rsp::R1b);
    assert_eq!(Command::SET_BUS_WIDTH_4.arg, 2);
    assert_eq!(Command::read_multiple(7).index, 18);
}

#[test]
fn each_response_maps_to_linux_sd_cfg2_type() {
    assert_eq!(Rsp::None.cfg2(), SD_RSP_TYPE_R0);
    assert_eq!(Rsp::R1.cfg2(), 0x01);
    assert_eq!(Rsp::R1b.cfg2(), 0x09);
    assert_eq!(Rsp::R2.cfg2(), 0x02);
    // R3 asks the reader not to check the CRC7.
    assert_eq!(Rsp::R3.cfg2() & SD_NO_CHECK_CRC7, SD_NO_CHECK_CRC7);
}

#[test]
fn ocr_and_if_cond() {
    assert!(if_cond_echoed(0x0000_01AA));
    assert!(!if_cond_echoed(0x0000_00AA));
    assert!(!ocr_ready(0x40FF_8000));
    assert!(ocr_ready(0xC0FF_8000) && ocr_high_capacity(0xC0FF_8000));
    assert!(ocr_ready(0x80FF_8000) && !ocr_high_capacity(0x80FF_8000));
}

#[test]
fn app_cmd_and_the_published_address_are_read_from_the_status() {
    // A version 1.x card's CMD55 status after the CMD8 it did not know:
    // ILLEGAL_COMMAND set, APP_CMD set. The command still counts.
    assert!(r1_app_cmd((1 << 22) | 0x120));
    assert!(!r1_app_cmd(0x0000_0100));
    assert_eq!(r6_rca(0xB368_0500), 0xB368);
}
