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

//! A driver's reply status in words. Besides the errnos, the drivers send
//! what the device itself said when it ended a command with an error, so a
//! failed install names the cause and not only a number:
//!
//! - NVMe: -(0x1000 | SCT << 8 | SC), the completion's status;
//! - SATA: -(0x1_0000 | ERR << 8 | STS), the disk's task file;
//! - eMMC host: -(0x100_0000 | CMD << 16 | Error Interrupt Status);
//! - eMMC card: -(0x200_0000 | CMD << 16 | the R1 error bit's number).
//!
//! Pure, for the host proofs.

use alloc::format;
use alloc::string::String;

pub fn describe(status: i32) -> String {
    let code = -(status as i64);
    match code {
        5 => String::from("I/O error"),
        6 => String::from("past the end of the disk"),
        19 => String::from("no disk behind the driver"),
        22 => String::from("a request the driver refused"),
        90 => String::from("a request of the wrong size"),
        110 => String::from("the device did not answer in time"),
        0x1000..=0x17ff => nvme(((code >> 8) & 0x7) as u8, (code & 0xff) as u8),
        0x1_0000..=0x1_ffff => ata(((code >> 8) & 0xff) as u8, (code & 0xff) as u8),
        0x100_0000..=0x1ff_ffff => emmc_host(((code >> 16) & 0x3f) as u8, (code & 0xffff) as u16),
        0x200_0000..=0x2ff_ffff => emmc_card(((code >> 16) & 0x3f) as u8, (code & 0x1f) as u8),
        _ => format!("status {status}"),
    }
}

fn nvme(sct: u8, sc: u8) -> String {
    let name = match (sct, sc) {
        (0, 0x01) => "invalid opcode",
        (0, 0x02) => "invalid field in command",
        (0, 0x04) => "data transfer error",
        (0, 0x05) => "aborted, power loss",
        (0, 0x06) => "internal error",
        (0, 0x07) => "aborted by request",
        (0, 0x0b) => "invalid namespace or format",
        (0, 0x13) => "PRP offset invalid",
        (0, 0x80) => "LBA out of range",
        (0, 0x81) => "capacity exceeded",
        (0, 0x82) => "namespace not ready",
        (1, 0x82) => "write to a read-only range",
        (2, 0x80) => "write fault",
        (2, 0x81) => "unrecovered read error",
        (2, 0x86) => "access denied",
        _ => "",
    };
    let sep = if name.is_empty() { "" } else { ", " };
    format!("NVMe status type {sct} code {sc:#04x}{sep}{name}")
}

fn ata(err: u8, sts: u8) -> String {
    let named = [
        (0x80, "interface CRC"),
        (0x40, "uncorrectable data"),
        (0x10, "sector not found"),
        (0x04, "command aborted"),
        (0x01, "address mark not found"),
    ];
    let mut s = format!("SATA status {sts:#04x} error {err:#04x}");
    if sts & 0x20 != 0 {
        s.push_str(", device fault");
    }
    for (bit, name) in named {
        if err & bit != 0 {
            s.push_str(", ");
            s.push_str(name);
        }
    }
    s
}

fn emmc_host(cmd: u8, err: u16) -> String {
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

fn emmc_card(cmd: u8, bit: u8) -> String {
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
