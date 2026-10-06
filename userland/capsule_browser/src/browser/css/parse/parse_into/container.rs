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

use crate::browser::css::calc::viewport;

use super::super::media_matches::media_matches;

/* An @container condition, judged against the viewport width: the
 * container's own inline size is known only after layout, and a page's
 * outer containers span the viewport. An optional container name leads
 * the condition; style() queries do not hold. Size features are the
 * media ones under their container names, and the range forms
 * (width > 40rem) turn into min-/max- features. */
pub(super) fn container_matches(cond: &str) -> bool {
    let c = cond.trim();
    let c = match c.find('(') {
        Some(0) => c,
        Some(p) if !c[..p].trim().eq_ignore_ascii_case("not") => c[p..].trim(),
        _ => c,
    };
    if c.to_ascii_lowercase().contains("style(") {
        return false;
    }
    media_matches(&as_media(c), viewport::width())
}

/* Rewrite each '(feature)' of the condition into media-query form. */
fn as_media(c: &str) -> String {
    let mut out = String::new();
    let mut rest = c;
    while let Some(open) = rest.find('(') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let close = after.find(')').unwrap_or(after.len());
        out.push_str(&format!("({})", feature(after[..close].trim())));
        rest = after.get(close + 1..).unwrap_or("");
    }
    out.push_str(rest);
    out
}

fn feature(f: &str) -> String {
    let f = f.replace("inline-size", "width").replace("block-size", "height");
    for (op, name) in [(">=", "min-"), ("<=", "max-"), (">", "min-"), ("<", "max-")] {
        if let Some((lhs, rhs)) = f.split_once(op) {
            if matches!(lhs.trim(), "width" | "height") {
                return format!("{}{}: {}", name, lhs.trim(), rhs.trim());
            }
        }
    }
    f
}
