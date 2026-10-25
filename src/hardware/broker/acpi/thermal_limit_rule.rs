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

//! Each processor's thermal limit: the percentage of full performance
//! passive cooling allows it (ACPI 6.5, 11.1.5), kept beside its _PPC for
//! the kernel's frequency control to read as a ceiling. A processor with
//! no entry runs unlimited; a limit of 100 removes its entry. Pure.

use alloc::vec::Vec;

pub const FULL: u64 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Placed {
    /// A limit set or moved.
    Set,
    /// The processor's entry removed: back to full.
    Cleared,
    Same,
    /// `max` processors already limited; nothing changed.
    Full,
    /// A percentage past 100.
    Bad,
}

pub fn place(list: &mut Vec<(u64, u8)>, uid: u64, pct: u64, max: usize) -> Placed {
    if pct > FULL {
        return Placed::Bad;
    }
    let at = list.iter().position(|(u, _)| *u == uid);
    match (at, pct == FULL) {
        (None, true) => Placed::Same,
        (Some(i), true) => {
            list.swap_remove(i);
            Placed::Cleared
        }
        (Some(i), false) if u64::from(list[i].1) == pct => Placed::Same,
        (Some(i), false) => {
            list[i].1 = pct as u8;
            Placed::Set
        }
        (None, false) if list.len() >= max || list.try_reserve(1).is_err() => Placed::Full,
        (None, false) => {
            list.push((uid, pct as u8));
            Placed::Set
        }
    }
}

/// The processor's ceiling, in percent of full performance.
pub fn ceiling(list: &[(u64, u8)], uid: u64) -> u8 {
    list.iter().find(|(u, _)| *u == uid).map_or(FULL as u8, |(_, p)| *p)
}
