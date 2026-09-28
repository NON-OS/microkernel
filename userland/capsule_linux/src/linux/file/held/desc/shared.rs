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

/* Whether another descriptor holds the same description. */

use super::handle::of;

/*
 * Whether a descriptor other than `fd` holds `fd`'s description: in this
 * process, or, when the family's view is lent, in any process of it.
 */
pub fn held_elsewhere(guest: &crate::linux::guest::Guest, fd: u64, d: u32) -> bool {
    let me = guest.pid;
    let here = guest
        .fds
        .iter()
        .enumerate()
        .any(|(i, o)| i as u64 != fd && o.is_open() && of(o) == Some(d));
    here || super::super::super::view::with(|v| {
        v.procs.iter().any(|p| p.kernel != me && p.fds.iter().any(|o| o.desc == Some(d)))
    })
}
