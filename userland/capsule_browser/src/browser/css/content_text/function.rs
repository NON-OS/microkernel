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

use super::args::{close_paren, unquote};

/* A content token other than a string. */
pub(super) enum Call<'a> {
    /* attr(name) with its fallback text. */
    Attr(&'a str, &'a str),
    /* counter(name, style) or counters(name, "sep", style). */
    Counter(&'a str, Option<&'a str>, &'a str),
    /* A quote keyword: opening or closing, and whether it draws a mark. */
    Quote(bool, bool),
    /* url(), a gradient, or anything this text has no use for. */
    Other,
}

/* The token at the start of `s` and the bytes it spans, at least one
 * whole character so the caller always moves on. */
pub(super) fn function(s: &str) -> (Call<'_>, usize) {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    let n = s.find(|c: char| !ident(c)).unwrap_or(s.len());
    let name = s[..n].to_ascii_lowercase();
    let first = s.chars().next().map_or(1, char::len_utf8);
    if !s[n..].starts_with('(') {
        let call = match name.as_str() {
            "open-quote" => Call::Quote(true, true),
            "close-quote" => Call::Quote(false, true),
            "no-open-quote" => Call::Quote(true, false),
            "no-close-quote" => Call::Quote(false, false),
            _ => Call::Other,
        };
        return (call, n.max(first));
    }
    let close = close_paren(s, n + 1);
    let args = &s[n + 1..close];
    let mut parts = args.split(',').map(str::trim);
    let arg0 = parts.next().unwrap_or("");
    let call = match name.as_str() {
        "attr" => Call::Attr(arg0.split_whitespace().next().unwrap_or(""), unquote(parts.next())),
        "counter" => Call::Counter(arg0, None, parts.next().unwrap_or("decimal")),
        "counters" => {
            let sep = unquote(parts.next());
            Call::Counter(arg0, Some(sep), parts.next().unwrap_or("decimal"))
        }
        _ => Call::Other,
    };
    (call, (close + 1).min(s.len()))
}
