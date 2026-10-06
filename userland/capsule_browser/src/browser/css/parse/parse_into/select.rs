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
use alloc::vec::Vec;

use crate::browser::css::rule::Rule;
use crate::browser::css::selector::Selector;

use super::super::selectors::parse_selectors;
use super::cost::selector_cost;
use super::element::element_of;

/* Selectors kept for one rule after nesting has multiplied its list. */
const MAX_PER_RULE: usize = 256;

/* The selectors of a rule, from its complex selectors as text, the Rule
 * flags they imply and an estimate of the bytes they keep. One selector that does not parse makes the whole
 * list invalid, and the result is empty so the rule drops, as in Chromium. */
pub(super) fn parse_list(texts: &[String]) -> (Vec<Selector>, u16, usize) {
    let (mut out, mut flags, mut cost) = (Vec::new(), 0u16, 0);
    for t in texts {
        let (host, code) = element_of(t);
        /* A bare ::placeholder is *::placeholder. */
        let host = if code != 0 && host.trim().is_empty() { "*" } else { host };
        let sels = parse_selectors(host);
        if sels.is_empty() {
            return (Vec::new(), 0, 0);
        }
        flags |= text_flags(t);
        for mut s in sels.into_iter().take(MAX_PER_RULE.saturating_sub(out.len())) {
            if s.element == 0 {
                s.element = code;
            }
            out.push(s);
            cost += selector_cost(t);
        }
    }
    (out, flags, cost)
}

/* The element states and attributes a selector reads, from its text. */
fn text_flags(t: &str) -> u16 {
    let t = t.to_ascii_lowercase();
    let mut f = 0;
    for (needle, bit) in
        [(":hover", Rule::HOVER), (":focus", Rule::FOCUS), (":active", Rule::ACTIVE)]
    {
        if t.contains(needle) {
            f |= bit;
        }
    }
    let value = ["[value", ":placeholder-shown", "valid", "range", ":blank"];
    if value.iter().any(|n| t.contains(n)) {
        f |= Rule::VALUE_ATTR;
    }
    f
}
