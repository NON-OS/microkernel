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

/*
 * What an owner's munmap does to one of its surfaces.
 *
 * A slot was freed only when its owner exited: an owner had no way to give
 * up the reference its register took, so every surface a long-lived owner
 * registered and unmapped kept one of the 256 slots for good. image_codec
 * registers one per decoded image and never exits, so a few hundred images
 * left no slot for any window on the machine. Unmapping the whole window
 * is now the owner giving the surface up.
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnUnmap {
    /// Nobody else maps it and all of it is gone: the slot is freed now.
    Free,
    /// Nobody else maps it and part stays mapped: it stops being attachable.
    Detach,
    /// Another process maps it and all of it is gone: its frames, and now its
    /// slot, wait for the last holder to let go.
    Orphan,
    /// Another process maps it and part stays mapped: the frames stay
    /// allocated and the surface stays live.
    Keep,
}

/// `attached`: a process other than the owner holds an attach record.
/// `whole`: the munmap covers every page of the owner's window.
pub fn on_unmap(attached: bool, whole: bool) -> OnUnmap {
    match (attached, whole) {
        (false, true) => OnUnmap::Free,
        (false, false) => OnUnmap::Detach,
        (true, true) => OnUnmap::Orphan,
        (true, false) => OnUnmap::Keep,
    }
}
