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

//! What a receiver's release leaves of the window its view of a surface
//! took. Pure, so kernel_proofs holds it.

/// What unmapping a receiver's view left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Window {
    /// The receiver's tables exist but their address space cannot be found:
    /// its record must stay, or the frames are freed under a view.
    Lost,
    /// Nothing to give back: the receiver owns the surface (its record is
    /// its own mapping), the surface left no frame list, the receiver's
    /// tables are gone, or a page of the window still maps something.
    Kept,
    /// The whole window, this many pages, maps nothing now: the attach that
    /// reserved it can have it back.
    Free(u64),
}

/// The window of a view `pages` long with `still` of its pages mapped once
/// the view came down. An owner's record is its own mapping and is never
/// given back here; a window any page of which still maps is kept, so the
/// allocator never hands out addresses a PTE still answers for.
pub fn window_back(owner: bool, pages: u64, still: usize) -> Window {
    if owner || pages == 0 || still != 0 {
        Window::Kept
    } else {
        Window::Free(pages)
    }
}
