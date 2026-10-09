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

//! Responses out of the written-back bytes, as sd_send_cmd_get_rsp reads
//! them: [check, response..., SD_STAT1].

use crate::regs::sd::*;
use crate::wire::{parse, ResponseError};

#[test]
fn an_r1_is_the_four_bytes_after_the_index_byte() {
    // CMD55 answered: index 0x37, status 0x00000120 (APP_CMD, ready).
    let bytes = [0x60, 0x37, 0x00, 0x00, 0x01, 0x20, 0x00];
    assert_eq!(parse(&bytes, SD_RSP_TYPE_R1).unwrap().words[0], 0x0000_0120);
}

#[test]
fn a_crc7_error_fails_an_r1_but_not_an_r3() {
    let bytes = [0x60, 0x3F, 0xC0, 0xFF, 0x80, 0x00, SD_CRC7_ERR];
    assert_eq!(parse(&bytes, SD_RSP_TYPE_R1), Err(ResponseError::Crc7));
    // ACMD41's R3 has no CRC; the OCR is taken as it is.
    assert_eq!(parse(&bytes, SD_RSP_TYPE_R3).unwrap().words[0], 0xC0FF_8000);
}

#[test]
fn start_bits_and_short_results_are_refused() {
    let bytes = [0x60, 0x80, 0, 0, 0, 0, 0];
    assert_eq!(parse(&bytes, SD_RSP_TYPE_R1), Err(ResponseError::StartBits));
    assert_eq!(parse(&bytes[..4], SD_RSP_TYPE_R1), Err(ResponseError::Short));
    assert!(parse(&[], SD_RSP_TYPE_R0).is_ok());
}

#[test]
fn an_r2_gets_the_dummy_crc_byte_in_its_lowest_bit() {
    let mut bytes = [0u8; 18];
    bytes[1] = 0x3F;
    for (i, b) in bytes[2..17].iter_mut().enumerate() {
        *b = i as u8 + 1;
    }
    let r = parse(&bytes, SD_RSP_TYPE_R2).unwrap();
    assert_eq!(r.words, [0x0102_0304, 0x0506_0708, 0x090A_0B0C, 0x0D0E_0F01]);
}
