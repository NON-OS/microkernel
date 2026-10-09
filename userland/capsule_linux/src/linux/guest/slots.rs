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

//! Which slot of a table a new pipe, eventfd, timerfd or signalfd takes.
//! Each table is this capsule's memory and grows only as far as the guest
//! holds descriptors on it: a slot no descriptor names any more is taken
//! again before the table grows. Pure, so the host proofs hold it.

/// The most slots one table holds: far more than the descriptors a family
/// can have open at once, so a table this full means a leak, refused ENFILE.
pub const MAX_SLOTS: usize = 4096;

/// The slot a new object takes in a table of `len`: the first one `free`
/// says nothing names, else a new one at the end, or None at MAX_SLOTS.
pub fn pick(len: usize, free: impl Fn(usize) -> bool) -> Option<usize> {
    (0..len.min(MAX_SLOTS)).find(|&i| free(i)).or_else(|| (len < MAX_SLOTS).then_some(len))
}
