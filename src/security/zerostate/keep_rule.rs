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

//! Who may turn keeping on, and what a store write needs, as pure rules.
//!
//! Keeping is the person's choice, which the policy service holds as
//! `Persistent`: setup sets it when the person picks a mode that keeps, and
//! the restore sets it again on a boot whose disk kept that choice. Only the
//! policy capsule the kernel spawned may tell the kernel so, by the pid it
//! was spawned as, and only to turn keeping on: nothing turns it off within a
//! boot, so a later push cannot make a kept boot's writes vanish half way.
//! Until then a write may only erase: every byte it changes must become
//! zero. A record an earlier boot kept can then still be wiped away, and
//! nothing new can be written down.
//!
//! No crate paths here, so the host proofs mount this file as it is.

/// The policy capsule as the kernel knows it: the pid it was spawned as,
/// and whether it is still running.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Policy {
    pub(crate) pid: u32,
    pub(crate) alive: bool,
}

/// True when `caller` asking to set keeping to `on` turns it on.
pub(crate) fn may_arm(caller: u32, policy: Policy, on: bool) -> bool {
    on && policy.alive && policy.pid != 0 && caller == policy.pid
}

/// Keeping after an ask: once on, it stays on for the boot.
pub(crate) fn after(armed: bool, granted: bool) -> bool {
    armed || granted
}

/// True when `new` may replace `old`, the store's bytes at the same place:
/// anything once keeping is on, and before that only a write that erases.
pub(crate) fn may_write(armed: bool, old: &[u8], new: &[u8]) -> bool {
    armed || erases_only(old, new)
}

/// Every byte `new` changes from `old` becomes zero, and they are the same
/// length, so nothing is written beyond what was read.
pub(crate) fn erases_only(old: &[u8], new: &[u8]) -> bool {
    old.len() == new.len() && old.iter().zip(new).all(|(o, n)| n == o || *n == 0)
}
