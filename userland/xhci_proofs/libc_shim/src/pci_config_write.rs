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

//! A masked write to the config space a test offered (`pci_config`):
//! recorded as tried, refused unless every bit it names is one the broker
//! admits, and landed only on a device that takes it.

use crate::log::{record, Call};
use crate::pci_config::SPACE;

const EPERM: i64 = -1;

pub(crate) fn write(offset: u32, value: u16) -> i64 {
    let (at, mask) = (offset & 0xFFFF, (offset >> 16) as u16);
    record(Call::PciConfig(at, mask, value));
    SPACE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(space) = s.as_mut() else { return EPERM };
        let admitted = space.admits.get(&at).copied().unwrap_or(0);
        if mask == 0 || mask & !admitted != 0 || value & !mask != 0 {
            return EPERM;
        }
        if space.takes {
            let cur = space.regs.get(&at).copied().unwrap_or(0);
            space.regs.insert(at, (cur & !(mask as u32)) | value as u32);
        }
        0
    })
}
