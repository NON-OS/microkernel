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

//! Moving between an output's levels: for the brightness keys, which the
//! firmware reports as Notify on the output (ACPI 6.5, B.7: 0x85 cycle,
//! 0x86 up, 0x87 down, 0x88 zero), and for a level asked from outside,
//! which is taken to the nearest one the output supports.

use super::levels::Levels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    Cycle,
    Up,
    Down,
    Zero,
}

pub fn change_of(notify: u8) -> Option<Change> {
    match notify {
        0x85 => Some(Change::Cycle),
        0x86 => Some(Change::Up),
        0x87 => Some(Change::Down),
        0x88 => Some(Change::Zero),
        _ => None,
    }
}

/// The level after `change`, from `current`; with no current level known,
/// from the AC level, else the top. Up and down stop at the ends; cycle
/// wraps to the bottom.
pub fn step(l: &Levels, current: Option<u8>, change: Change) -> u8 {
    let (bottom, top) = (l.steps[0], l.steps[l.steps.len() - 1]);
    let cur = current.or(l.on_ac).unwrap_or(top);
    let above = l.steps.iter().copied().find(|&s| s > cur);
    match change {
        Change::Up => above.unwrap_or(top),
        Change::Down => l.steps.iter().rev().copied().find(|&s| s < cur).unwrap_or(bottom),
        Change::Zero => bottom,
        Change::Cycle => above.unwrap_or(bottom),
    }
}

/// The supported level nearest `percent`; the lower on a tie.
pub fn nearest(l: &Levels, percent: u8) -> u8 {
    let d = |s: &u8| s.abs_diff(percent);
    l.steps.iter().copied().min_by_key(|s| (d(s), *s)).unwrap_or(percent)
}
