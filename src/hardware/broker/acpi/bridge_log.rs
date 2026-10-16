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
//! should a real `_PSx` need one, the log names it. Past REMEMBERED of them
//! one line says the rest go untold (rule::tell), so firmware asking in a
//! loop, over any number of offsets, never fills the log.

use super::rule::{tell, Tell};
use crate::sys::serial::Line;
use alloc::vec::Vec;
use spin::Mutex;

const REMEMBERED: usize = 64;

static SEEN: Mutex<Vec<(u64, u16)>> = Mutex::new(Vec::new());

/// `key` is the function as the region lines print it.
pub(super) fn refused(key: u64, at: u16, width: u8, value: u32) {
    let mut seen = SEEN.lock();
    match tell(seen.contains(&(key, at)), seen.len(), REMEMBERED) {
        Tell::Quiet => return,
        Tell::New => seen.push((key, at)),
        Tell::Last => {
            // No function has this key, so it only marks the line as said.
            seen.push((u64::MAX, 0));
            return Line::new().str(b"[ACPI] further refused bridge writes are not logged").end();
        }
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
