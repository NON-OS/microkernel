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

/// Words a line may hold. It was 8, and a ninth was dropped without a word:
/// `echo` and eight words printed seven.
pub const MAX_ARGS: usize = 64;

/// What a line with more words than that says; it runs nothing.
pub const TOO_MANY: &[u8] = b"too many words on one line, 64 at most, so nothing ran";

pub struct Argv<'a> {
    pub argv: [&'a [u8]; MAX_ARGS],
    pub argc: usize,
    /// The line held more than `MAX_ARGS` words.
    pub cut: bool,
}
