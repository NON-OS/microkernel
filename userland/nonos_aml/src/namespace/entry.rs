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

//! One named object.

use super::Kind;

/// An index into the namespace's entries. The root is entry 0.
pub type Id = u32;
pub const ROOT: Id = 0;

/// A NameSeg's four characters.
pub type Seg = [u8; 4];

/// The objects section 5.3.1 says exist before any table is loaded, and the
/// predefined ones (section 5.7) the operating system supplies itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Predefined {
    Root,
    Gpe,
    Pr,
    Sb,
    Si,
    Tz,
    Gl,
    Os,
    Osi,
    Rev,
}

/// Where an object was defined: a node in a loaded table's arena, or by the
/// operating system before any table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Table { table: u16, node: u32 },
    Predefined(Predefined),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub seg: Seg,
    /// The root is its own parent.
    pub parent: Id,
    pub kind: Kind,
    pub origin: Origin,
    /// Bumped each time the slot is reused, so a reference taken to an
    /// object a method made, and kept after the method returned, cannot
    /// reach whatever took the slot next.
    pub gen: u32,
    pub(super) live: bool,
    /// Children in the order they were defined, as a linked list, so a walk
    /// in definition order (section 6.5.1's _INI order) needs no sort.
    pub(super) first: Option<Id>,
    pub(super) last: Option<Id>,
    pub(super) next: Option<Id>,
}
