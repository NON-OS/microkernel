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

use crate::browser::css::apply::apply_decl;
use crate::browser::css::computed::Computed;
use crate::browser::css::selector::Pseudo;

use super::super::selectors::parse_selectors;
use super::ops::split_ops;
use super::scan::find_top;

/* Display values the display applier lays out. */
const DISPLAYS: &str = "none block inline inline-block flex inline-flex grid inline-grid contents \
                        list-item table table-row table-cell inline-table";

/* An @supports condition, answered by this engine: a declaration holds
 * when an applier of the cascade claims the property (and, for display,
 * lays out the value); selector() holds when the selector parses and can
 * match; not, and, or combine; anything else does not hold. */
pub(super) fn supports(cond: &str) -> bool {
    eval(cond.trim(), 0)
}

fn eval(c: &str, depth: u32) -> bool {
    if depth > 8 {
        return false;
    }
    let lower = c.get(..4).map(|h| h.to_ascii_lowercase());
    if matches!(lower.as_deref(), Some("not ") | Some("not(")) {
        return !eval(c[3..].trim(), depth + 1);
    }
    let (parts, any) = split_ops(c);
    if parts.len() > 1 {
        let mut hits = parts.iter().map(|p| eval(p, depth + 1));
        return if any { hits.any(|h| h) } else { hits.all(|h| h) };
    }
    if let Some(sel) = c.strip_prefix("selector(").and_then(|s| s.strip_suffix(')')) {
        let sels = parse_selectors(sel);
        let never = |s: &crate::browser::css::selector::Selector| {
            s.key.pseudo.iter().any(|p| matches!(p, Pseudo::Never))
        };
        return !sels.is_empty() && !sels.iter().any(never);
    }
    let Some(inner) = c.strip_prefix('(').and_then(|s| s.strip_suffix(')')) else {
        return false;
    };
    let colon = find_top(inner, b":");
    if colon >= inner.len() {
        return eval(inner.trim(), depth + 1);
    }
    let (name, value) = (inner[..colon].trim().to_ascii_lowercase(), inner[colon + 1..].trim());
    if name.starts_with("--") {
        return true;
    }
    let first = value.split_whitespace().next().unwrap_or("");
    let claimed = apply_decl(&mut Computed::root(), &name, value, 16);
    claimed
        && (name != "display" || DISPLAYS.split_whitespace().any(|d| d.eq_ignore_ascii_case(first)))
}
