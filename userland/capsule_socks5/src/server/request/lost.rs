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

//! Telling a lost conversation from a new one.

/// The first number of a conversation, after its reset.
pub const FIRST_SEQ: u32 = 1;

/// Whether exchange `seq` asks for a conversation this proxy lost: not the
/// first of one, while no conversation is `held` for its stream and no
/// answer is `kept` for that number.
pub fn lost(seq: u32, held: bool, kept: bool) -> bool {
    seq != FIRST_SEQ && !held && !kept
}
