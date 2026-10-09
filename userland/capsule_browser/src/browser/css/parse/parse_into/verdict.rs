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
use alloc::vec;

use crate::browser::css::decl::Decl;
use crate::browser::css::rule::Rule;

use super::cx::Cx;

/* Record how a viewport condition (@media, @container) came out, as a
 * COND data rule, and pass the verdict on. A resize re-checks these
 * records: when none flips, the parsed rules stand and only layout
 * runs again. */
pub(super) fn verdict(cx: &mut Cx, kind: &str, cond: &str, holds: bool) -> bool {
    let decls = vec![
        Decl::new(String::from(kind), String::from(cond), false),
        Decl::new(String::from("holds"), String::from(if holds { "1" } else { "0" }), false),
    ];
    cx.statement(decls, Rule::COND);
    holds
}
