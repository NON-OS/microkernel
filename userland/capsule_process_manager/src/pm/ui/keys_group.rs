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

//! How the overlay groups the bindings.
//!
//! The headings are what a reader is actually trying to do, not the internals
//! the keys happen to touch: someone hunting for a key wants "narrow the table",
//! not "filter enum".

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Group {
    Move,
    Sort,
    Filter,
    Act,
}

impl Group {
    pub fn label(self) -> &'static [u8] {
        match self {
            Group::Move => b"MOVE AROUND",
            Group::Sort => b"ORDER THE TABLE",
            Group::Filter => b"NARROW THE TABLE",
            Group::Act => b"ACT ON A PROCESS",
        }
    }
}
