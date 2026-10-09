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

use alloc::borrow::Cow;
use alloc::string::String;

/// Whitespace as the tree builder sees it: tab, LF, FF, CR and space. A CR
/// can still arrive here from a character reference.
pub fn is_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\x0C' | '\r' | ' ')
}

/// Split a run of characters into its leading whitespace and the rest, for
/// the modes that treat the two differently.
pub fn split_space(s: Cow<'_, str>) -> (Cow<'_, str>, Cow<'_, str>) {
    let n = s.len() - s.trim_start_matches(is_space).len();
    match s {
        Cow::Borrowed(b) => (Cow::Borrowed(&b[..n]), Cow::Borrowed(&b[n..])),
        Cow::Owned(mut o) => {
            let rest = o.split_off(n);
            (Cow::Owned(o), Cow::Owned(rest))
        }
    }
}

/// Drop the first character of a run, the newline after <pre>.
pub fn drop_first(s: &mut Cow<'_, str>) {
    match s {
        Cow::Borrowed(b) => *b = &b[1..],
        Cow::Owned(o) => {
            o.remove(0);
        }
    }
}

/// Only the whitespace of a run, for the frameset modes that drop the rest.
pub fn only_space(s: &str) -> String {
    s.chars().filter(|&c| is_space(c)).collect()
}

/// Whether a run is all whitespace.
pub fn all_space(s: &str) -> bool {
    s.chars().all(is_space)
}
