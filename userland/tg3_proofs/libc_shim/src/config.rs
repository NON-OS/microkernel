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

//! A function's configuration space, per test thread, with the broker's
//! calls over it: reads of 1, 2 or 4 bytes inside the 256-byte header, and
//! 16-bit writes recorded in order. The broker's own rule on which bits a
//! write may move is held by the tests against the kernel's quirk_bits.

use std::cell::RefCell;

const SPACE: usize = 256;
const EINVAL: i64 = -22;

thread_local! {
    static SPACE_BYTES: RefCell<[u8; SPACE]> = const { RefCell::new([0; SPACE]) };
    static WRITES: RefCell<Vec<(u32, u16)>> = const { RefCell::new(Vec::new()) };
}

/// Puts `value` at `offset` as the device would hold it, little-endian.
pub fn config_set(offset: u32, value: u32, width: u32) {
    SPACE_BYTES.with(|s| {
        let mut s = s.borrow_mut();
        for i in 0..width as usize {
            s[offset as usize + i] = (value >> (8 * i)) as u8;
        }
    });
}

/// Every 16-bit write made on this thread, as (offset, value), in order.
pub fn config_writes() -> Vec<(u32, u16)> {
    WRITES.with(|w| w.borrow().clone())
}

pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, width: u32) -> i64 {
    if !matches!(width, 1 | 2 | 4) || offset as usize + width as usize > SPACE {
        return EINVAL;
    }
    SPACE_BYTES.with(|s| {
        let s = s.borrow();
        (0..width as usize).fold(0i64, |v, i| v | (s[offset as usize + i] as i64) << (8 * i))
    })
}

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    if offset as usize + 2 > SPACE {
        return EINVAL;
    }
    WRITES.with(|w| w.borrow_mut().push((offset, value)));
    config_set(offset, value as u32, 2);
    0
}
