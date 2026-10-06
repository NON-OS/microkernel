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

mod api;
mod append;
mod flips;
mod parsed;
mod policy;
mod query;
mod text;

use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::compute::{cascade, Author, Inputs, Styled};
use super::rule::Rule;
use super::rule_index::RuleIndex;
use text::TextKey;

/* Parsed sheets kept between relayouts, and the last cascade. The rules
 * are tagged with the CSS text they came from and the viewport the parse
 * ran at (@media blocks are chosen while parsing); a sheet appended to
 * that text is parsed alone and joins them. The last styles are kept
 * with the document fingerprint and viewport they were computed for, so
 * a relayout that cannot change a style reuses them. */
pub struct CssCache {
    text: TextKey,
    viewport: (u32, u32),
    ua: (Vec<Rule>, RuleIndex),
    author: Author,
    memo: Option<Memo>,
    /* Fingerprint and viewport of the last layout. */
    laid: Option<(u64, (u32, u32))>,
    /* The <noscript> policy's inputs: QuickJS on, and whether the page's
     * scripts ran without failing, None when that is not known. */
    js: (bool, Option<bool>),
}

/* One cascade's result and what it was computed from. */
struct Memo {
    print: u64,
    viewport: (u32, u32),
    js: (bool, Option<bool>),
    styled: Styled,
}

/* Cascade with a persistent parse cache at the page viewport `viewport`
 * (width, height in px), which vw/vh units and @media features resolve
 * against. The parse is reused while the CSS text and viewport stand. */
pub fn compute_cached(
    dom: &Dom,
    author_css: &str,
    viewport: (u32, u32),
    cache: &mut Option<CssCache>,
) -> Styled {
    let _held = super::calc::viewport::enter(viewport.0, viewport.1);
    let c = parsed::ensure(cache, author_css, viewport);
    cascade(dom, &Inputs { ua: (&c.ua.0, &c.ua.1), author: &c.author, js: c.js })
}
