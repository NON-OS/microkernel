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

//! The waiter rules, pure over the list, so the host proofs hold them.

use alloc::vec::Vec;

/// `pid` waits on slot `idx`; once only, however often it arms.
pub(super) fn arm_in(list: &mut Vec<(u32, u32)>, idx: u32, pid: u32) {
    if !list.contains(&(idx, pid)) {
        list.push((idx, pid));
    }
}

pub(super) fn disarm_in(list: &mut Vec<(u32, u32)>, idx: u32, pid: u32) {
    list.retain(|&w| w != (idx, pid));
}

/// Every waiter on slot `idx`, taken off the list.
pub(super) fn take_in(list: &mut Vec<(u32, u32)>, idx: u32) -> Vec<u32> {
    let woken = list.iter().filter(|w| w.0 == idx).map(|w| w.1).collect();
    list.retain(|w| w.0 != idx);
    woken
}

pub(super) fn forget_in(list: &mut Vec<(u32, u32)>, pid: u32) {
    list.retain(|w| w.1 != pid);
}
