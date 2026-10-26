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

//! The two numbers the kernel heap's growth rests on, pure so kernel_proofs
//! holds them: the second region's size for a machine's RAM, and whether a
//! process's page is refused for the free heap left.

/// An eighth of `ram`, at least `min` and at most `max`, in whole `page`s.
pub(crate) fn second_region_size(ram: u64, min: usize, max: usize, page: usize) -> usize {
    let eighth = (ram / 8).min(max as u64) as usize;
    eighth.max(min) / page * page
}

/// A page mapped for a process is refused while the heap's free bytes are
/// under the floor; the kernel's own mappings never are.
pub(crate) fn user_page_refused(user: bool, free: usize, floor: usize) -> bool {
    user && free < floor
}
