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

//! Persistent goes one way within a boot, as the kernel's keep latch does.
//!
//! The kernel is told only when keeping turns on and never turns it off, and
//! vfs asks the kernel, not this service. A later set to false would leave
//! this store saying the boot keeps nothing while the disk still took writes,
//! so it is refused, and the two answers stay the same.
//!
//! No crate paths here, so the host proofs mount this file as it is.

/// True when Persistent may become `value` while it holds `was`.
pub(crate) fn may_set(was: bool, value: bool) -> bool {
    value || !was
}
