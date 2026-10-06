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

use core::cell::Cell;

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::decl::Decl;
use crate::browser::css::selector::Selector;

use super::cx::Cx;
use super::select::parse_list;
use super::{nesting, parse_into, MAX_DEPTH};

/* A style rule whose block is being read: its complex selectors as text,
 * which nested rules build on, and its parsed list until the first run of
 * declarations takes it. */
pub(in crate::browser::css::parse) struct Parent {
    pub texts: Vec<String>,
    pub flags: u16,
    first: Cell<Option<(Vec<Selector>, usize)>>,
}

impl Parent {
    /* The rule as a parent, or None when its selector list is invalid:
     * then the rule and everything nested in it drop. */
    pub fn new(prelude: &str, outer: Option<&Parent>) -> Option<Parent> {
        let texts = nesting::resolve(prelude, outer.map(|p| p.texts.as_slice()));
        let (sels, flags, cost) = parse_list(&texts);
        if sels.is_empty() {
            return None;
        }
        Some(Parent { texts, flags, first: Cell::new(Some((sels, cost))) })
    }
}

/* A block: an at-rule's, or a qualified rule's declarations, in order
 * around any nested rules. */
pub(super) fn block(head: &str, body: &str, cx: &mut Cx, depth: u32, outer: Option<&Parent>) {
    if head.starts_with('@') {
        super::at_rule::at_rule(head, body, cx, depth, outer);
    } else if let Some(me) = Parent::new(head, outer).filter(|_| depth < MAX_DEPTH) {
        parse_into(body, cx, depth + 1, Some(&me));
    }
}

/* Declarations read since the last nested rule become one rule of the
 * parent's selectors, so a declaration after a nested rule keeps its place
 * in the cascade order after that rule. */
pub(super) fn flush(own: &mut Vec<Decl>, p: Option<&Parent>, cx: &mut Cx) {
    let Some(p) = p.filter(|_| !own.is_empty()) else { return };
    let (sels, cost) = match p.first.take() {
        Some(s) => s,
        None => {
            let (sels, _, cost) = parse_list(&p.texts);
            (sels, cost)
        }
    };
    cx.push(sels, core::mem::take(own), p.flags, cost);
}
