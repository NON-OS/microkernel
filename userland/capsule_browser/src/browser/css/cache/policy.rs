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

use crate::browser::css::compute::noscript_shows;
use crate::browser::dom::Dom;

use super::CssCache;

/* Ancestors a link looks through for an enclosing <noscript>. */
const MAX_UP: usize = 512;

impl CssCache {
    /* Whether `dom`'s <noscript> content renders, with QuickJS on or off
     * (`js_on`) and the page's scripts having run or failed, None when
     * not known (`scripts_ok`): the UA sheet's <noscript> policy, which
     * the cascade applies and a fetcher asks before loading a stylesheet
     * linked inside <noscript>. */
    pub fn noscript_shows(dom: &Dom, js_on: bool, scripts_ok: Option<bool>) -> bool {
        noscript_shows(dom, (js_on, scripts_ok))
    }

    /* Whether a stylesheet linked by a child of node `parent` loads: it
     * does unless it sits inside a <noscript> whose content does not
     * render (rustdoc's noscript.css, for readers without scripts). */
    pub fn sheet_loads(dom: &Dom, parent: usize, js_on: bool) -> bool {
        let mut at = parent;
        for _ in 0..MAX_UP {
            let Some(n) = dom.nodes.get(at) else { return true };
            if n.tag == "noscript" {
                return Self::noscript_shows(dom, js_on, None);
            }
            if at == 0 {
                return true;
            }
            at = n.parent;
        }
        true
    }
}
