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

//! The text types the desktop clipboard is carried as. The desktop holds
//! text alone, so nothing else a guest offers reaches it, and only text is
//! offered back. Pure, so the host proofs hold it.

/// UTF-8 first: a guest's selection is read in the first of these its
/// source offered. The desktop's text is UTF-8, and is written as it is
/// whichever of them a guest asks for.
pub const TEXT: [&[u8]; 5] =
    [b"text/plain;charset=utf-8", b"UTF8_STRING", b"text/plain", b"TEXT", b"STRING"];

/// Whether `mime` is one of TEXT. A charset is matched in either case, as
/// toolkits write it both ways.
pub fn is_text(mime: &[u8]) -> bool {
    TEXT.iter().any(|t| t.eq_ignore_ascii_case(mime))
}

/// The type to read a selection offering `mimes` in, as its source spelled
/// it; None when it offers no text.
pub fn text_of(mimes: &[alloc::vec::Vec<u8>]) -> Option<&[u8]> {
    TEXT.iter().find_map(|t| mimes.iter().find(|m| t.eq_ignore_ascii_case(m))).map(|m| m.as_slice())
}
