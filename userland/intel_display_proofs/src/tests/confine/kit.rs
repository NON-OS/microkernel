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
//! Arming the watch with a rig's frames, and what a confined table is.

use crate::model::confine::names;
use crate::model::frames::CURSOR_IOVA;
use crate::model::Gen9;

/// Firmware's surface in every rig: one page, mapped over its whole run.
pub const FIRMWARE: (u64, u64) = (0x9000_0000, 0x9000_1000);

/// The watch armed with surfaces at `bases`, `pages` pages each, and the
/// cursor image the driver keeps for its life.
pub fn arm(m: &mut Gen9, bases: &[u64], pages: usize) {
    let len = pages as u64 * 4096;
    let mut held: Vec<_> = bases.iter().map(|&b| (b, b + len)).collect();
    held.push((CURSOR_IOVA, CURSOR_IOVA + 16384));
    m.arm(held);
}

/// Present entries naming a page neither held nor firmware's.
pub fn unheld(m: &Gen9) -> Vec<(u32, u64)> {
    let held = m.confine.held.as_deref().unwrap_or(&[]);
    let named = |pte: u64| names(held, pte) || names(&[FIRMWARE], pte);
    let present = m.ggtt.iter().enumerate().filter(|e| e.1 & 1 != 0);
    present.filter(|e| !named(*e.1)).map(|(i, &p)| (i as u32, p)).collect()
}

/// No stray written, none left stale, and every entry held or firmware's.
pub fn confined(m: &Gen9, tag: &str) {
    let c = &m.confine;
    assert!(c.held.is_some(), "{tag}: the watch armed");
    assert!(c.strays.is_empty(), "{tag}: strays {:x?}", &c.strays[..c.strays.len().min(3)]);
    assert!(c.stale.is_empty(), "{tag}: stale {:x?}", &c.stale[..c.stale.len().min(3)]);
    let left = unheld(m);
    assert!(left.is_empty(), "{tag}: unheld {:x?}", &left[..left.len().min(3)]);
}
