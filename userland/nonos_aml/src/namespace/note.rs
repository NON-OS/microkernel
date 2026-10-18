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

//! What loading noticed and did not stop for, kept for the capsule to log.

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoteKind {
    /// A name defined again in the same scope; the first definition stands.
    Redefined,
    /// A Scope naming no object; its TermList was not loaded.
    ScopeMissing,
    /// A definition whose parent scope does not exist.
    ParentMissing,
    /// An Alias naming no object.
    AliasMissing,
    /// A load-time statement the interpreter could not run.
    Exec,
    /// A FieldList whose fields run past 2^32 bits; the rest were not made.
    Layout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub kind: NoteKind,
    pub table: u16,
    /// The table offset of the object the note is about.
    pub at: u32,
}

/// At most `CAP` notes are kept, so a table full of duplicates cannot grow
/// the log without bound; the rest are counted.
#[derive(Debug, Default)]
pub struct Notes {
    pub list: Vec<Note>,
    pub dropped: u32,
}

impl Notes {
    pub const CAP: usize = 256;

    pub fn push(&mut self, kind: NoteKind, table: u16, at: u32) {
        if self.list.len() < Self::CAP {
            self.list.push(Note { kind, table, at });
        } else {
            self.dropped = self.dropped.saturating_add(1);
        }
    }
}
