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

mod flags;

use alloc::rc::Rc;
use alloc::vec::Vec;

use super::decl::Decl;
use super::selector::Selector;
use flags::decl_flags;

/* A style rule, or, with no selectors, a statement the cascade reads as
 * data: an '@layer a, b;' order (LAYER_ORDER, one decl per layer named),
 * an @property registration (PROPERTY, its descriptors as decls), or the
 * verdict on a viewport condition (COND: the at-rule name and condition,
 * then whether it held), which a resize re-checks. */
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub decls: Vec<Decl>,
    /* The dotted @layer path the rule sits in; None outside any layer. */
    pub layer_name: Option<Rc<str>>,
    /* That layer's cascade rank, set when the sheet is ranked: higher wins
     * among normal declarations, and UNLAYERED outranks every layer. */
    pub layer: u16,
    pub flags: u16,
    /* Bytes the parse estimated this rule keeps, counted against the
     * page's budget, Rule::MAX_BYTES. */
    pub cost: u32,
}

impl Rule {
    pub const UNLAYERED: u16 = u16::MAX;
    pub const IMPORTANT: u16 = 1;
    pub const CUSTOM: u16 = 2;
    pub const FONT_SIZE: u16 = 4;
    pub const LAYER_ORDER: u16 = 8;
    pub const PROPERTY: u16 = 16;
    /* A value in vw, vh, vmin or vmax: a resize changes its computed px. */
    pub const VP_UNITS: u16 = 32;
    /* The selectors test :hover, :focus (any form) or :active. */
    pub const HOVER: u16 = 64;
    pub const FOCUS: u16 = 128;
    pub const ACTIVE: u16 = 256;
    /* The selectors read the value attribute, which typing changes. */
    pub const VALUE_ATTR: u16 = 512;
    /* counter-reset, counter-increment, counter-set or a counter() value. */
    pub const COUNTERS: u16 = 1024;
    pub const COND: u16 = 2048;
    /* A content declaration: only then does a ::before or ::after exist. */
    pub const CONTENT: u16 = 4096;

    pub fn new(selectors: Vec<Selector>, decls: Vec<Decl>, flags: u16) -> Rule {
        let mut flags = flags;
        for d in &decls {
            flags |= decl_flags(d);
        }
        Rule { selectors, decls, layer_name: None, layer: Rule::UNLAYERED, flags, cost: 0 }
    }
}
