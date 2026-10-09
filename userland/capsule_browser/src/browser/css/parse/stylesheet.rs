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

use alloc::vec::Vec;

use crate::browser::css::rule::Rule;

use super::parse_into::{parse_into, Cx};
use super::strip_comments::strip_comments;

/* The rules of a sheet in source order, with @media, @supports and
 * @container blocks already chosen for the current viewport, @layer
 * paths recorded on each rule, and '@layer' order and @property
 * statements kept as selector-less data rules. Each rule's src is its
 * byte offset in the comment-stripped text. The rules kept stay within
 * the page budget, Rule::MAX_BYTES. */
pub fn parse(css: &str) -> Vec<Rule> {
    let src = strip_comments(css);
    let mut rules: Vec<Rule> = Vec::new();
    let origin = src.as_ptr() as usize;
    let (left, bytes) = (Rule::MAX_SELECTORS, Rule::MAX_BYTES);
    let mut cx = Cx { rules: &mut rules, origin, layer: None, left, bytes };
    parse_into(&src, &mut cx, 0, None);
    rules
}
