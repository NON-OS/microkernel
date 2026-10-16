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

//! Where the register window was mapped from, for the status reply.

use core::sync::atomic::{AtomicU32, Ordering};

/// Which BAR the register window came from, and the low half of the address it
/// was mapped at. rtw88 hardcodes bar_id 2 for this chip; this driver takes the
/// first MMIO BAR the broker reports, so a change in what the broker reports
/// would silently move the window somewhere that still answers reads. On a
/// machine with no serial console the panel is the only place to check.
pub(super) static BAR_INDEX: AtomicU32 = AtomicU32::new(0xFFFF_FFFF);
pub(super) static WINDOW_VA: AtomicU32 = AtomicU32::new(0);

/// The BAR index the window was taken from and the low 32 bits of its address.
/// An index of `0xFFFFFFFF` means no window has been mapped.
pub fn window() -> (u32, u32) {
    (BAR_INDEX.load(Ordering::Relaxed), WINDOW_VA.load(Ordering::Relaxed))
}
