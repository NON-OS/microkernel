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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* Index of the <body> element, or 0 (the document root) when a page has none. */
pub(crate) fn body_id(dom: &Dom) -> usize {
    dom.nodes.iter().position(|n| n.kind == NodeKind::Element && n.tag == "body").unwrap_or(0)
}

/* Index of the root element, <html>; a fragment with none roots at <body>. */
pub(crate) fn root_id(dom: &Dom) -> usize {
    let html = |n: &crate::browser::dom::node::Node| n.kind == NodeKind::Element && n.tag == "html";
    dom.nodes.iter().position(html).unwrap_or_else(|| body_id(dom))
}
