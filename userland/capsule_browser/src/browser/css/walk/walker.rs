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

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::budget::MatchBudget;
use crate::browser::css::compute::Styles;
use crate::browser::css::grid_spec::GridSpec;
use crate::browser::css::matching::Siblings;
use crate::browser::css::pseudo_style::PseudoText;
use crate::browser::css::rule::Rule;
use crate::browser::css::rule_index::{Entry, RuleIndex};
use crate::browser::css::vars::Props;
use crate::browser::dom::Dom;

use super::counters::Counters;
use super::order::Hit;

/* A parsed sheet and its selector index. */
#[derive(Clone, Copy)]
pub(in crate::browser::css) struct Sheet<'a> {
    pub rules: &'a [Rule],
    pub index: &'a RuleIndex,
}

/* What the cascade walk writes, per node. */
pub(in crate::browser::css) struct Out {
    pub styles: Styles,
    pub bg_images: Vec<Option<String>>,
    pub svg_paint: Vec<Option<Box<str>>>,
    pub grids: Vec<Option<Box<GridSpec>>>,
    /* Boxed while the walk grows the list: a push or a sort moves a
     * pointer, not the style. */
    pub pseudos: Vec<(u32, Box<PseudoText>)>,
}

/* One cascade over a tree: its inputs, per-element scratch space reused
 * from element to element, and the output being built. */
pub(in crate::browser::css) struct Walker<'a> {
    pub dom: &'a Dom,
    pub sib: &'a Siblings,
    pub ua: Sheet<'a>,
    pub author: Sheet<'a>,
    /* The author selectors that style pseudo-elements. */
    pub pseudo: &'a RuleIndex,
    pub props: &'a Props,
    pub budget: MatchBudget,
    /* <noscript> content renders (the page's scripts cannot stand in). */
    pub noscript: bool,
    /* CSS counters, kept only when a sheet uses them. */
    pub counters: Option<Counters>,
    /* <body link>, the colour legacy pages give their links. */
    pub link: Option<String>,
    pub buckets: Vec<&'a [Entry]>,
    pub ua_hits: Vec<Hit>,
    pub author_hits: Vec<Hit>,
    pub out: Out,
}
