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

//! BRIGHTNESS and SET_BRIGHTNESS over `Brightness`: every output that
//! answers is a panel; its levels are 1..=100 percent (0 is left out, so no
//! request blanks a panel by accident: 0 goes to 1), each the share of the
//! PWM's period nearest that percent.

use super::brightness::Brightness;
use super::brightness_wire::{identity, EIO, ENOENT};

const LEVELS: core::ops::RangeInclusive<u8> = 1..=100;

/// The panels into `out`; ENOENT with an empty body when none.
pub fn list<B: Brightness>(b: &mut B, outputs: u16, out: &mut [u8]) -> (i32, usize) {
    let mut at = 1;
    let mut count = 0u8;
    for o in 0..outputs {
        let Ok((level, max)) = b.brightness(o) else { continue };
        let now = ((level as u64 * 100 + max as u64 / 2) / max.max(1) as u64).min(100) as u8;
        let n = LEVELS.len();
        let Some(slot) = out.get_mut(at..at + 6 + n) else { break };
        slot[..4].copy_from_slice(&identity(o).to_le_bytes());
        (slot[4], slot[5]) = (now, n as u8);
        for (d, l) in slot[6..].iter_mut().zip(LEVELS) {
            *d = l;
        }
        (at, count) = (at + 6 + n, count + 1);
    }
    match (count, out.first_mut()) {
        (0, _) | (_, None) => (ENOENT, 0),
        (c, Some(first)) => {
            *first = c;
            (0, at)
        }
    }
}

/// Every panel, or `panel` alone, to the level nearest `pct`.
pub fn set<B: Brightness>(b: &mut B, outputs: u16, pct: u8, panel: Option<u32>) -> i32 {
    let pct = pct.clamp(*LEVELS.start(), *LEVELS.end()) as u64;
    let mut found = false;
    let mut failed = false;
    for o in (0..outputs).filter(|o| panel.is_none_or(|p| p == identity(*o))) {
        let Ok((_, max)) = b.brightness(o) else { continue };
        found = true;
        let level = ((pct * max as u64 + 50) / 100) as u32;
        failed |= b.set_brightness(o, level).is_err();
    }
    match (found, failed) {
        (false, _) => ENOENT,
        (true, true) => EIO,
        (true, false) => 0,
    }
}
