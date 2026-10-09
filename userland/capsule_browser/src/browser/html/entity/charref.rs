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

use super::decoded::push_pair;
use super::lookup::{legacy_prefix, lookup, LEGACY_MAX, NAME_MAX};
use super::numeric_ref::numeric_ref;
use super::push_decoded;

/// Decode the character reference at the start of `rest`, the text right
/// after an ampersand, the way the tokenizer does (13.2.5.77 to 13.2.5.84).
///
/// What it stands for is appended to `out` and the number of bytes of `rest`
/// it used is returned. When no reference starts here a lone "&" is appended
/// and nothing is used, so what follows is read as ordinary text: `AT&T`
/// stays `AT&T` and `?a=1&b=2` keeps its query.
pub fn charref(rest: &str, in_attr: bool, out: &mut String) -> usize {
    match rest.as_bytes().first() {
        Some(b'#') => numeric_ref(rest, out),
        Some(c) if c.is_ascii_alphanumeric() => named(rest, in_attr, out),
        _ => {
            out.push('&');
            0
        }
    }
}

/// The longest name in the table the input starts with. A name written with
/// its semicolon has to be the whole run of letters and digits; one of the
/// legacy names may stop anywhere inside it.
fn named(rest: &str, in_attr: bool, out: &mut String) -> usize {
    let b = rest.as_bytes();
    let run = b.iter().take(NAME_MAX + 1).take_while(|c| c.is_ascii_alphanumeric()).count();
    let closed = b.get(run) == Some(&b';');
    if let Some(pair) = closed.then(|| lookup(&rest[..run])).flatten() {
        push_pair(out, pair);
        return run + 1;
    }
    let len = legacy_prefix(&b[..run.min(LEGACY_MAX)]);
    if len == 0 {
        out.push('&');
        return 0;
    }
    /*
     * Inside an attribute a legacy name followed by "=" or a letter is left
     * as written: `?x=1&copy=2` is a query string, not a copyright sign.
     */
    let next = b.get(len).copied().unwrap_or(b' ');
    if in_attr && (next == b'=' || next.is_ascii_alphanumeric()) {
        out.push('&');
        out.push_str(&rest[..len]);
        return len;
    }
    push_decoded(out, &rest[..len]);
    len
}
