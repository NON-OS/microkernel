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

/// One table row per name: the name without its semicolon, the character it
/// stands for, and a second character for the few names that expand to two
/// (written `first+second`), NUL otherwise. A macro keeps the data dense
/// enough to read as a table rather than one tuple per line.
macro_rules! rows {
    ($($name:literal $first:literal $(+ $second:literal)?)*) => {
        &[$(($name, super::rows::cp($first), super::rows::cp(0 $(+ $second)?))),*]
    };
}

pub(super) use rows;

/// A table code point as a character. Evaluated while the table is built, so
/// a row that is not a Unicode scalar value fails the build rather than
/// reaching a page.
pub(super) const fn cp(value: u32) -> char {
    match char::from_u32(value) {
        Some(c) => c,
        None => panic!("entity table row is not a Unicode scalar value"),
    }
}
