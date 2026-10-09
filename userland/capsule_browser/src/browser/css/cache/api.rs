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

use crate::browser::css::rule::Rule;
use crate::browser::dom::Dom;

use crate::browser::css::compute_cached;

use super::{flips::flips, parsed, CssCache, Memo};

impl CssCache {
    /* Bring the kept styles up to date for `dom` at `viewport`. When they
     * are recomputed, the CSS text they came from comes back; None when
     * the kept ones stand. `print` is the document's fingerprint
     * with and without form field values: typing restyles only when a
     * selector reads a value. `css` produces the CSS text, and is called
     * only when the kept styles cannot stand. `js` is (QuickJS on, the
     * page's scripts ran without failing if known), the <noscript>
     * policy's inputs. */
    pub fn restyle(
        cache: &mut Option<CssCache>,
        dom: &Dom,
        css: &mut dyn FnMut() -> String,
        viewport: (u32, u32),
        print: (u64, u64),
        js: (bool, Option<bool>),
    ) -> Option<String> {
        let held = crate::browser::css::calc::viewport::enter(viewport.0, viewport.1);
        if let Some(c) = cache.as_mut() {
            let key = c.style_key(print);
            let vp_units = c.author.flags & Rule::VP_UNITS != 0;
            if let Some(m) = c.memo.as_mut().filter(|m| (m.print, m.js) == (key, js)) {
                if m.viewport == viewport {
                    return None;
                }
                if !vp_units && !flips(&c.author.rules) {
                    (m.viewport, c.viewport) = (viewport, viewport);
                    return None;
                }
            }
        }
        let text = css();
        if cache.as_ref().map(|c| c.js) != Some(js) {
            parsed::ensure(cache, &text, viewport).js = js;
        }
        drop(held);
        let styled = compute_cached(dom, &text, viewport, cache);
        let c = cache.as_mut()?;
        let print = c.style_key(print);
        c.memo = Some(Memo { print, viewport, js, styled });
        Some(text)
    }
}
