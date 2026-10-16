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

//! A bridge write MkAcpiPci refused, logged once per (function, offset):
//! should a real `_PSx` need one, the log names it, and firmware asking in
//! a loop never fills the log.

use crate::sys::serial::Line;
use alloc::vec::Vec;
use spin::Mutex;

const REMEMBERED: usize = 64;

static SEEN: Mutex<Vec<(u64, u16)>> = Mutex::new(Vec::new());

/// `key` is the function as the region lines print it.
pub(super) fn refused(key: u64, at: u16, width: u8, value: u32) {
    let mut seen = SEEN.lock();
    if seen.contains(&(key, at)) {
        return;
    }
    // Past the bound, keep logging rather than go quiet on a new offset.
    if seen.len() < REMEMBERED {
        seen.push((key, at));
    }
    Line::new()
        .str(b"[ACPI] pci write refused seg.bus.dev.fn=")
        .hex(key)
        .str(b" offset=")
        .hex(u64::from(at))
        .str(b" width=")
        .dec(u64::from(width))
        .str(b" value=")
        .hex(u64::from(value))
        .str(b": bridge bus number, window or bus master")
        .end();
}
