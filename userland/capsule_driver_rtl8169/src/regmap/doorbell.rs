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

use super::doorbell_of;
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Tell the part a descriptor is ready on the normal-priority ring.
pub fn ring(regs: &Regs, ver: MacVersion) {
    let bell = doorbell_of(ver);
    // SAFETY: 0x38 and 0x90 lie inside the register window of every chip
    // this driver starts (at least 0x100 bytes); the driver owns the part.
    unsafe {
        if bell.wide {
            regs.w16(bell.offset, bell.value);
        } else {
            regs.w8(bell.offset, bell.value as u8);
        }
    }
}
