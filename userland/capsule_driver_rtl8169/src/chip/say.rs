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

use super::{Chip, ChipError};
use crate::log::Line;

/// "rtl8169: chip RTL_GIGA_MAC_VER_46 xid 0x541 (RTL8168h/8111h)".
pub fn found(chip: &Chip) {
    Line::new("rtl8169: chip RTL_GIGA_MAC_VER_")
        .dec(chip.ver.0 as u32, 2)
        .text(if chip.extended { " ext xid " } else { " xid " })
        .hex(chip.xid)
        .text(" (")
        .text(chip.name)
        .text(")")
        .send();
}

pub fn refused(err: &ChipError) {
    match err {
        ChipError::ReadFailed => Line::new("rtl8169: TxConfig reads 0xffffffff, not started"),
        ChipError::Unknown { xid, extended } => Line::new(if *extended {
            "rtl8169: unknown chip ext xid "
        } else {
            "rtl8169: unknown chip xid "
        })
        .hex(*xid)
        .text(", not started"),
        ChipError::ExtendedOutOfWindow => {
            Line::new("rtl8169: extended chip id at 0x60b0 is past the mapped bar, not started")
        }
        ChipError::BarTooSmall(len) => Line::new("rtl8169: an 8125 needs its 64 KiB bar, mapped ")
            .hex(*len as u32)
            .text(", not started"),
        ChipError::Unsupported(chip) => Line::new("rtl8169: chip RTL_GIGA_MAC_VER_")
            .dec(chip.ver.0 as u32, 2)
            .text(" (")
            .text(chip.name)
            .text(") not supported by this driver, not started"),
    }
    .send();
}
