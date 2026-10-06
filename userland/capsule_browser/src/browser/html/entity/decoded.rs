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

use alloc::string::String;

use super::lookup::lookup;
use super::numeric::numeric;

/// Append what the reference `&entity;` names, the name given without its
/// ampersand and semicolon.
///
/// These used to fold onto ASCII lookalikes: an em dash arrived as two
/// hyphens, a copyright sign as three characters in brackets, a non
/// breaking space as an ordinary one. The bundled face carries all of them,
/// so the substitution only ever cost fidelity and changed how lines wrap.
///
/// Anything unrecognised is written back out as it arrived. A page carrying
/// a reference this does not know is better read with the source visible
/// than with the text around it silently dropped. The tokenizer's own rules,
/// which differ for unknown names and control numbers, are in `charref`.
pub fn push_decoded(out: &mut String, entity: &str) {
    if let Some(c) = numeric(entity) {
        out.push(c);
        return;
    }
    match lookup(entity) {
        Some(pair) => push_pair(out, pair),
        None => {
            out.push('&');
            out.push_str(entity);
            out.push(';');
        }
    }
}

/// Append what a table row stands for: one character, or two where the
/// second is not NUL.
pub(super) fn push_pair(out: &mut String, (first, second): (char, char)) {
    out.push(first);
    if second != '\0' {
        out.push(second);
    }
}
