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
 * is now the project giving the surface up.
 *
 * A guest's pin is given up at the first unmap that meets any of it:
 * peer_unmap unpins it before the pages change, and the guest's supervisor
 * unmaps a span a MiB at a time (capsule_linux guest/mem_unmap.rs). Kept on
 * a part unmap, as an owner's surface is, a pinned buffer over a MiB came
 * down in parts that each kept their frames, and once the compositor let
 * the pin go the slot went and those frames were never freed: a pinned
 * Telegram window lost its buffer's frames each time it made new ones.
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
    /// allocated and the surface stays live. Never a pin's.
    Keep,
}

/// `attached`: a process other than the project holds an attach record.
/// `whole`: the munmap covers every page of the project's window. `pin`:
/// the project is a guest and the surface its pin. A pin another process
/// maps becomes an orphan on a part unmap too: what the guest unmaps then
/// and later is freed once that process lets go, and what it still maps
/// stays its own until it unmaps it or exits (exit_owned.rs).
pub fn on_unmap(attached: bool, whole: bool, pin: bool) -> OnUnmap {
    match (attached, whole) {
        (false, true) => OnUnmap::Free,
        (false, false) => OnUnmap::Detach,
        (true, true) => OnUnmap::Orphan,
        (true, false) if pin => OnUnmap::Orphan,
        (true, false) => OnUnmap::Keep,
    }
}
