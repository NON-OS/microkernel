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

mod layer_tree;
mod layers;
mod noscript;
mod pseudos;
mod sheets;
mod start;
mod styles;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::grid_spec::GridSpec;
pub(super) use noscript::shows as noscript_shows;
pub use pseudos::Pseudos;
pub(super) use sheets::{Author, Inputs};
pub use styles::Styles;

/* The cascade output, per node: the computed style, the background image
 * (a url or a gradient), the resolved SVG paint style of SVG elements, the
 * named-grid data and the pseudo-elements of the elements that have any. */
pub struct Styled {
    pub styles: Styles,
    pub bg_images: Vec<Option<String>>,
    pub svg_paint: Vec<Option<Box<str>>>,
    pub grids: Vec<Option<Box<GridSpec>>>,
    pub pseudos: Pseudos,
}

/* Parse `author_css` and cascade it over `dom`: the direct path the
 * render harness uses; the capsule goes through CssCache. */
#[cfg(feature = "harness")]
pub fn compute(dom: &Dom, author_css: &str) -> Styled {
    let ua = super::ua::ua_rules();
    let ua_index = super::rule_index::RuleIndex::build(&ua, false);
    let author = Author::new(super::parse::parse(author_css));
    cascade(dom, &Inputs { ua: (&ua, &ua_index), author: &author, js: (true, None) })
}

/* Cascade parsed sheets over the tree. Author matching runs under a work
 * budget, so a hostile sheet degrades styling from the end of the
 * document instead of stalling the page. */
pub(super) fn cascade(dom: &Dom, input: &Inputs) -> Styled {
    let sib = super::matching::Siblings::table(dom);
    let mut w = input.walker(dom, &sib, noscript::shows(dom, input.js));
    let root = super::vars::VarScope::root();
    let top = super::computed::Computed::root();
    super::walk::walk(&mut w, 0, (&top, 0, &root), 0);
    let out = w.out;
    Styled {
        styles: out.styles,
        bg_images: out.bg_images,
        svg_paint: out.svg_paint,
        grids: out.grids,
        pseudos: Pseudos::from(out.pseudos),
    }
}
