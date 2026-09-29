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

use crate::browser::css::selector::Pseudo;

use super::cx::Cx;
use super::dir::is_rtl;
use super::form::form_matches;
use super::has::has;
use super::lang::lang_matches;
use super::misc::{defined, empty, is_link, is_open, is_root};
use super::nth_of::nth_of;
use super::selector::test;
use super::structural::structural;
use super::user::user_matches;

/* One pseudo-class at one element. Selector-list arguments are matched
 * with the element as their subject, each through the full matcher with its
 * own combinators; they share the call's step budget. */
pub(super) fn pseudo_matches(cx: &Cx, id: usize, p: &Pseudo) -> bool {
    match p {
        Pseudo::Never => false,
        Pseudo::Matches(list) => list.iter().any(|s| test(cx, id, s)),
        Pseudo::Not(list) => !list.iter().any(|s| test(cx, id, s)),
        Pseudo::Has(list) => has(cx, id, list),
        Pseudo::HasAnchor => cx.anchor.get() == id,
        Pseudo::NthChildOf(a, b, list) => nth_of(cx, id, (*a, *b), list, false),
        Pseudo::NthLastChildOf(a, b, list) => nth_of(cx, id, (*a, *b), list, true),
        Pseudo::Empty => empty(cx.dom, id),
        Pseudo::Root => is_root(cx.dom, id),
        Pseudo::Scope if cx.scope == 0 => is_root(cx.dom, id),
        Pseudo::Scope => cx.scope == id,
        Pseudo::AnyLink => is_link(cx.dom, id),
        Pseudo::Lang(want) => lang_matches(cx.dom, id, want),
        Pseudo::Dir(rtl) => is_rtl(cx, id) == *rtl,
        Pseudo::Defined => defined(cx.dom, id),
        Pseudo::Open => is_open(cx.dom, id),
        Pseudo::Form(f) => form_matches(cx, id, *f),
        Pseudo::User(u) => user_matches(cx.dom, id, *u),
        _ => structural(cx, id, p),
    }
}
