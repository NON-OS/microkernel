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

use crate::browser::css::parse::parse;
use crate::browser::css::rule::Rule;

/* Whether any @media or @container verdict the parse recorded in `rules`
 * comes out otherwise at the viewport now published (the caller holds
 * its guard). Each condition is judged by the parser itself, on a block
 * holding nothing but that condition. */
pub(super) fn flips(rules: &[Rule]) -> bool {
    rules.iter().filter(|r| r.flags & Rule::COND != 0).any(|r| {
        let (Some(kind), Some(held)) = (r.decls.first(), r.decls.get(1)) else {
            return true;
        };
        let probe = parse(&format!("@{} {}{{}}", kind.name, kind.value));
        let now = probe.first().and_then(|p| p.decls.get(1)).is_some_and(|d| d.value == "1");
        now != (held.value == "1")
    })
}
