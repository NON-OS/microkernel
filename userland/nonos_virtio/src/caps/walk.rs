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

//! The capability list walk, bounded against every list a broken or hostile
//! function can present: no list at all, a pointer into the standard header,
//! a pointer that loops back, and a list longer than the area can hold.

use super::layout::{CAP_FIRST, CAP_NEXT, CAP_POINTER, MAX_CAPS, PCI_STATUS, STATUS_CAP_LIST};
use crate::pci::ConfigSpace;

/// Pointers are dword aligned; the low two bits are reserved.
const POINTER_MASK: u8 = 0xFC;

/// Yields the offset of each capability, in list order.
pub struct CapWalk<'a> {
    cfg: &'a ConfigSpace,
    next: u8,
    /// One bit per config-space dword already visited: a pointer is masked
    /// to 0xFC, so 64 bits cover every place one can name.
    seen: u64,
    steps: usize,
}

impl<'a> CapWalk<'a> {
    pub fn new(cfg: &'a ConfigSpace) -> Self {
        let listed = cfg.u16_at(PCI_STATUS).is_some_and(|s| s & STATUS_CAP_LIST != 0);
        let head = if listed { cfg.u8_at(CAP_POINTER).unwrap_or(0) & POINTER_MASK } else { 0 };
        Self { cfg, next: head, seen: 0, steps: 0 }
    }
}

impl Iterator for CapWalk<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let ptr = self.next;
        if ptr < CAP_FIRST || self.steps >= MAX_CAPS {
            return None;
        }
        let bit = 1u64 << (ptr >> 2);
        if self.seen & bit != 0 {
            return None;
        }
        self.seen |= bit;
        self.steps += 1;
        self.next = self.cfg.u8_at(ptr as usize + CAP_NEXT).unwrap_or(0) & POINTER_MASK;
        Some(ptr as usize)
    }
}
