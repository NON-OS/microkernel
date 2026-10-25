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

//! What the machine remembers of the link once autojoin is done.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Rejoin {
    /// The link was associated when last seen.
    pub(super) up: bool,
    /// Attempts left in the rejoin under way; 0 when none is.
    pub(super) left: u8,
    pub(super) backoff_ms: i64,
    /// When the link is next looked at.
    pub(super) next_ms: i64,
}

impl Rejoin {
    pub(super) const NEW: Self = Self { up: false, left: 0, backoff_ms: 0, next_ms: 0 };
}
