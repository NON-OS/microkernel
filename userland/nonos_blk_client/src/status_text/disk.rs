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

//! What an NVMe or SATA disk itself said when it ended a command with an
//! error, in words (status_text.rs).

use alloc::format;
use alloc::string::String;

pub fn nvme(sct: u8, sc: u8) -> String {
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

pub fn ata(err: u8, sts: u8) -> String {
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
