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

use alloc::string::{String, ToString};

/* What a click on a link does. */
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkAction {
    Navigate(String),
    /* A fragment of the document already shown: scroll to it, no fetch. */
    Anchor(String),
    /* javascript:, mailto:, tel: and every other scheme the browser cannot
     * open: the click does nothing rather than replacing the page. */
    Ignore,
}

/* `resolved` is the link joined against the page base; `current` is the
 * address of the document on screen. */
pub fn link_action(current: &str, resolved: &str) -> LinkAction {
    let r = resolved.trim();
    let web = ["http://", "https://"]
        .iter()
        .any(|p| r.get(..p.len()).is_some_and(|h| h.eq_ignore_ascii_case(p)));
    if !web {
        return LinkAction::Ignore;
    }
    if let Some((page, frag)) = r.split_once('#') {
        if same_document(current, page) {
            return LinkAction::Anchor(frag.to_string());
        }
    }
    LinkAction::Navigate(r.to_string())
}

/* Whether two addresses name the same document, fragments aside. */
pub fn same_document(a: &str, b: &str) -> bool {
    !a.is_empty() && doc(a) == doc(b)
}

fn doc(s: &str) -> &str {
    s.split('#').next().unwrap_or("")
}
