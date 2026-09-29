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

use crate::browser::dom;

use super::copy_children::copy_children;
use super::ctx::Ctx;

/// innerHTML setter: parse the markup as the contents of the target, as the
/// fragment parsing algorithm does, and graft the result under it, replacing
/// its children. The target is the context, so "<tr><td>" set on a tbody
/// is a row, and nothing wraps the fragment in html, head and body.
pub(super) fn graft_html(ctx: &mut Ctx, id: usize, html: &str) {
    if id >= ctx.dom.nodes.len() {
        return;
    }
    let context = ctx.dom.nodes[id].context_tag();
    let frag = dom::parse_fragment(html.as_bytes(), &context);
    ctx.dom.nodes[id].children.clear();
    copy_children(ctx.dom, &frag, 0, id, 0);
    ctx.dirty = true;
}
