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

//! The rest of the device's config space, as a test offers it: registers
//! that read back, and the bits of each the broker would let the driver
//! change. A write names its register and, in bits 16 to 31, the bits it
//! changes (the broker's masked form); one outside what is offered is
//! refused as the kernel refuses it, and one the device ignores (`takes`
//! false) is admitted but never lands. Unoffered, every read is refused.

use std::cell::RefCell;
use std::collections::HashMap;

const EINVAL: i64 = -22;

pub(crate) struct Space {
    pub(crate) regs: HashMap<u32, u32>,
    pub(crate) admits: HashMap<u32, u16>,
    pub(crate) takes: bool,
}

thread_local! {
    pub(crate) static SPACE: RefCell<Option<Space>> = const { RefCell::new(None) };
}

/// This thread's device answers `regs` (offset, value) to reads, and the
/// broker admits writes within `admits` (offset, mask).
pub fn offer_config(regs: &[(u32, u32)], admits: &[(u32, u16)], takes: bool) {
    let space = Space {
        regs: regs.iter().copied().collect(),
        admits: admits.iter().copied().collect(),
        takes,
    };
    SPACE.with(|s| *s.borrow_mut() = Some(space));
}

/// What the register at `offset` holds now, if it was offered.
pub fn config_value(offset: u32) -> Option<u32> {
    SPACE.with(|s| s.borrow().as_ref().and_then(|s| s.regs.get(&offset).copied()))
}

pub(crate) fn read(offset: u32, width: u32) -> i64 {
    let mask = match width {
        1 => 0xFF,
        2 => 0xFFFF,
        4 => u32::MAX,
        _ => return EINVAL,
    };
    config_value(offset).map_or(EINVAL, |v| (v & mask) as i64)
}
