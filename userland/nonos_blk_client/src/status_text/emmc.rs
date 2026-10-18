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

//! What an eMMC host controller or card said when a command ended in an
//! error, in words (status_text.rs).

use alloc::format;
use alloc::string::String;

pub fn emmc_host(cmd: u8, err: u16) -> String {
    let named = [
        (1 << 0, "command timeout"),
        (1 << 1, "command CRC"),
        (1 << 2, "command end bit"),
        (1 << 3, "command index"),
        (1 << 4, "data timeout"),
        (1 << 5, "data CRC"),
        (1 << 6, "data end bit"),
        (1 << 7, "current limit"),
        (1 << 8, "auto CMD12/23"),
        (1 << 9, "ADMA"),
    ];
    let mut s = format!("eMMC host error {err:#06x} on CMD{cmd}");
    for (bit, name) in named {
        if err & bit != 0 {
            s.push_str(", ");
            s.push_str(name);
        }
    }
    s
}

pub fn emmc_card(cmd: u8, bit: u8) -> String {
    let name = match bit {
        31 => "OUT_OF_RANGE",
        30 => "ADDRESS_ERROR",
        29 => "BLOCK_LEN_ERROR",
        28 => "ERASE_SEQ_ERROR",
        27 => "ERASE_PARAM",
        26 => "WP_VIOLATION",
        25 => "CARD_IS_LOCKED",
        24 => "LOCK_UNLOCK_FAILED",
        23 => "COM_CRC_ERROR",
        22 => "ILLEGAL_COMMAND",
        21 => "CARD_ECC_FAILED",
        20 => "CC_ERROR",
        19 => "ERROR",
        7 => "SWITCH_ERROR",
        _ => "an error",
    };
    format!("eMMC card reported {name} on CMD{cmd}")
}
