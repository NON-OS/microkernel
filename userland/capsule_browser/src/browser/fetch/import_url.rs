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

mod media_part;
use media_part::media_part;

/* Pull the target out of an @import prelude, the text between the @import
 * token and its semicolon: url("x"), url('x'), url(x) or a bare "x" / 'x'
 * string. Also returns the media query list that follows it, with any
 * layer() or supports() condition skipped (layers do not decide loading). */
pub(super) fn import_url(prelude: &str) -> Option<(String, &str)> {
    let p = prelude.trim();
    let (target, rest) = if let Some(r) = p.strip_prefix("url(") {
        let end = r.find(')')?;
        let inner = r[..end].trim();
        let inner = inner
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| inner.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(inner);
        (inner, &r[end + 1..])
    } else {
        let quote = *p.as_bytes().first()?;
        if quote != b'"' && quote != b'\'' {
            return None;
        }
        let end = p[1..].find(quote as char)?;
        (&p[1..1 + end], &p[2 + end..])
    };
    if target.is_empty() {
        return None;
    }
    Some((String::from(target), media_part(rest)))
}
