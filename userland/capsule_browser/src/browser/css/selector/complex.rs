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

use super::simple::{Simple, Step};

/* One complex selector read right to left: the key (the rightmost compound,
 * the element being matched), then each compound further left with the
 * combinator that joins it to its right-hand neighbour. */
#[derive(Clone)]
pub struct Selector {
    pub key: Simple,
    pub ancestors: Vec<Step>,
    /* 0 plain, 1 ::before, 2 ::after. Codes from 3 up name pseudo-elements
     * that are parsed but not styled, so the cascade passes over them. */
    pub element: u8,
    /* Specificity packed as (ids << 20) | (classes << 10) | types, counted
     * from the selector as written. */
    pub spec: u32,
    /* Filter bits every candidate must find among its ancestors: the tag,
     * id and classes of each compound joined by a child or descendant
     * combinator, which are always ancestors of the key. */
    pub anc_bits: [u64; 2],
    /* Matching asks sibling positions or walks siblings, so a query over
     * many elements builds the sibling table first. */
    pub positional: bool,
}
