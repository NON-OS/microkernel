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
use alloc::rc::Rc;

use super::super::computed::Computed;
use super::super::content_text::content_text;
use super::super::vars::{At, VarScope};
use super::super::walk::{custom_scope, Hit, Order, Styling, Walker};
use super::PseudoText;

/* Cascade one pseudo-element from its rules. A ::before or ::after with
 * content none or normal, or display none, generates no box. */
pub(super) fn one(w: &mut Walker, id: usize, host: &Computed, scope: &Rc<VarScope>, group: &[Hit]) {
    let kind = group[0].elem;
    let order = Order { ua: (&[], &[]), hints: &[], author: (w.author.rules, group), inline: &[] };
    let scope = custom_scope(&order, scope, w.props, id);
    let mut st = Styling::new(host, At { scope: &scope, props: w.props, id });
    st.run(&order);
    let node = &w.dom.nodes[id];
    let text = st.content.as_deref().and_then(|v| content_text(v, node, w.counters.as_mut()));
    let generated = matches!(kind, PseudoText::BEFORE | PseudoText::AFTER);
    if generated && (text.is_none() || st.c.display_none) {
        return;
    }
    let p = PseudoText { kind, text, style: st.c, bg_image: st.bg };
    w.out.pseudos.push((id as u32, Box::new(p)));
}
