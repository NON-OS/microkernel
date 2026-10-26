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

//! The store as a pack's swap sees it (swap.rs), and what it answers.

/// Why a pack did not go in or out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Held {
    /// A run holds the pack open: close it, then ask again.
    InUse,
    /// The store would not write, rename or remove it.
    Unwritten,
}

/// What the store says to an unlink: gone (or never there), held open by
/// another process, or not removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unlinked {
    Gone,
    Busy,
    Failed,
}

/// The store, as a pack's swap uses it. A rename never writes over a name.
pub trait Shelf {
    fn write(&mut self, at: &[u8], bytes: &[u8]) -> bool;
    fn unlink(&mut self, at: &[u8]) -> Unlinked;
    fn rename(&mut self, from: &[u8], to: &[u8]) -> bool;
}
