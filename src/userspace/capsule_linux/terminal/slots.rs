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

//! Which terminal-run slot is free, reserved for a spawn under way, or held
//! by a live run, and for which terminal. Taken from the syscall, spawn and
//! exit paths, so the lock masks interrupts while it is held.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::roles::TERMINAL;
use crate::sys::sync::IrqMutex;

pub(super) enum Slot {
    Free,
    /// A spawn is under way for `parent`; `argv` goes to the process when
    /// it is admitted, before it first runs.
    Reserved {
        parent: u32,
        argv: Vec<String>,
    },
    Held {
        parent: u32,
        pid: u32,
    },
}

const FREE: Slot = Slot::Free;
pub(super) static SLOTS: IrqMutex<[Slot; TERMINAL.len()]> = IrqMutex::new([FREE; TERMINAL.len()]);

/// Reserve the first free slot for `parent`, or `None` when all are taken.
pub(super) fn reserve(parent: u32, argv: Vec<String>) -> Option<usize> {
    let mut slots = SLOTS.lock();
    let i = slots.iter().position(|s| matches!(s, Slot::Free))?;
    slots[i] = Slot::Reserved { parent, argv };
    Some(i)
}

/// Give slot `i` back after a spawn that did not admit a process.
pub(super) fn cancel(i: usize) {
    let mut slots = SLOTS.lock();
    if let Some(s @ Slot::Reserved { .. }) = slots.get_mut(i) {
        *s = Slot::Free;
    }
}

/// Move reserved slot `i` to held by `pid`, handing back its run request,
/// or `None` when slot `i` was not reserved.
pub(super) fn hold(i: usize, pid: u32) -> Option<Vec<String>> {
    let mut slots = SLOTS.lock();
    let slot = slots.get_mut(i)?;
    match core::mem::replace(slot, Slot::Free) {
        Slot::Reserved { parent, argv } => {
            *slot = Slot::Held { parent, pid };
            Some(argv)
        }
        other => {
            *slot = other;
            None
        }
    }
}
