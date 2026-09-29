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

use alloc::format;
use alloc::string::String;

/* Bytes one joined selector may take. */
const MAX_JOINED: usize = 4 << 10;

/* One parent and one nested selector joined, or None when the result
 * would pass MAX_JOINED; the size is known before anything is copied. */
pub(super) fn join(parent: &str, child: &str) -> Option<String> {
    if child.is_empty() {
        /* An empty entry keeps the list invalid, as it is on its own. */
        return Some(String::new());
    }
    let amps = child.bytes().filter(|&b| b == b'&').count();
    if child.len() + amps.max(1) * parent.len() + 1 > MAX_JOINED {
        return None;
    }
    if amps > 0 {
        return Some(child.replace('&', parent));
    }
    if let Some(rest) = strip_scope(child) {
        return Some(format!("{parent}{rest}"));
    }
    Some(format!("{parent} {child}"))
}

/* ":scope" leading a selector in an @scope block names the scope root. */
fn strip_scope(child: &str) -> Option<&str> {
    let head = child.get(..6)?;
    head.eq_ignore_ascii_case(":scope").then(|| &child[6..])
}
