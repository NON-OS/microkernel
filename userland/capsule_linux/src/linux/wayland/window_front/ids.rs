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
//! A window brought to `Scene.win`, to be acted on; the one there goes

//! The window manager id a guest window opens with. The window manager
//! keys a window by its owner and this id, so two windows of one guest must
//! never share one: taken from the kernel's surface handle, whose low word
//! is the same for every surface, a dialog was given its parent's id and
//! every key and click meant for it went to the parent.

/// Window ids for guest windows, clear of the native apps' fixed ones.
pub const GUEST_WINDOWS: u32 = 0x4C4E_0000;
/// Window ids for guest popups in windows of their own, clear of both.
pub const GUEST_POPUPS: u32 = 0x4C50_0000;

/// The lowest id in `base`'s range none of `used` is; None when all are
/// taken.
pub fn free_id(base: u32, used: &[u32]) -> Option<u32> {
    (1..=0xFFFF).map(|n| base | n).find(|id| !used.contains(id))
}
