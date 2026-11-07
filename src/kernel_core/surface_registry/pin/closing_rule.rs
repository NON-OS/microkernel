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
 * An exiting process's surfaces between keep_at_exit and the slot release
 * (K15b), pure. keep_at_exit decided which frames to keep from the attach
 * records it saw; an attach after it, before release_owned_by_pid dropped
 * the slots, mapped frames the exit then freed. From keep_at_exit on, those
 * surfaces refuse every attach. Proven in kernel_proofs::surface_closing.
 */

extern crate alloc;

use alloc::vec::Vec;

/// Surfaces of exiting processes: (exiting pid, handle).
pub(crate) type Closing<H> = Vec<(u32, H)>;

/// `pid` exits: its surfaces `handles` refuse attaches from now on.
pub(crate) fn close<H: Copy>(list: &mut Closing<H>, pid: u32, handles: &[H]) {
    list.extend(handles.iter().map(|&h| (pid, h)));
}

/// Whether `handle` may still be attached.
pub(crate) fn admits<H: PartialEq>(list: &Closing<H>, handle: H) -> bool {
    !list.iter().any(|(_, h)| *h == handle)
}

/// `pid`'s slots are gone, their handles stale: its entries go too.
pub(crate) fn forget<H>(list: &mut Closing<H>, pid: u32) {
    list.retain(|(p, _)| *p != pid);
}
