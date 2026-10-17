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

//! What acs.rs reads of a function: a bridge's secondary bus, and its ACS
//! words, with what a hop must redirect turned on first (K11).

use super::acs_rule::enabling;
use super::acs_tree::Bdf;
use crate::drivers::pci::config::{read32_unchecked, read_extended32, write_extended32};

const ACS_ID: u32 = 0x000D;

/// A type 1 header's secondary and subordinate bus.
pub(super) fn buses((b, d, f): Bdf) -> Option<(u8, u8)> {
    let bridge = (read32_unchecked(b, d, f, 0x0C) >> 16) & 0x7F == 1;
    let numbers = read32_unchecked(b, d, f, 0x18);
    bridge.then_some(((numbers >> 8) as u8, (numbers >> 16) as u8))
}

/// The ACS capability and control words, with `needed` turned on first
/// where the capability offers it all.
pub(super) fn redirect((b, d, f): Bdf, needed: u16) -> Option<(u16, u16)> {
    let at = acs_offset((b, d, f))? + 4;
    let words = |v: u32| (v as u16, (v >> 16) as u16);
    let (cap, ctrl) = words(read_extended32(b, d, f, at));
    if let Some(on) = enabling(cap, ctrl, needed).filter(|&on| on != ctrl) {
        write_extended32(b, d, f, at, u32::from(cap) | u32::from(on) << 16);
    }
    Some(words(read_extended32(b, d, f, at)))
}

/// The ACS extended capability's offset, walking at most 64 headers.
fn acs_offset((b, d, f): Bdf) -> Option<u16> {
    let mut at = 0x100u16;
    for _ in 0..64 {
        let head = read_extended32(b, d, f, at);
        if head == !0 || head == 0 {
            return None;
        }
        if head & 0xFFFF == ACS_ID {
            return Some(at);
        }
        at = ((head >> 20) & 0xFFC) as u16;
        if at < 0x100 {
            return None;
        }
    }
    None
}
