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

/// One pass over the structure block. Each step consumes at least one token,
/// so a walk ends within `structure.len() / 4` steps whatever the blob holds,
/// and nesting is held to `MAX_DEPTH` so no caller's per-depth table can be
/// overrun. The stepping itself is in `step.rs`.
pub struct Walker<'a> {
    pub(super) structure: &'a [u8],
    pub(super) strings: &'a [u8],
    pub(super) cursor: usize,
    pub(super) depth: usize,
    pub(super) done: bool,
}

impl<'a> Walker<'a> {
    pub fn new(structure: &'a [u8], strings: &'a [u8]) -> Self {
        Self::at(structure, strings, 0, 0)
    }

    /// A walk that starts at `cursor` with `depth` nodes already open, which
    /// is how a node's own properties and children are revisited.
    pub fn at(structure: &'a [u8], strings: &'a [u8], cursor: usize, depth: usize) -> Self {
        Self { structure, strings, cursor, depth, done: false }
    }

    /// Nodes open after the last event: 1 inside the root.
    pub fn depth(&self) -> usize {
        self.depth
    }
}
