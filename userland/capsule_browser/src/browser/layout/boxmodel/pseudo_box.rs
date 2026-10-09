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

mod node;

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::PseudoText;

use super::tree::BoxNode;
use super::walk::Walk;
use node::pseudo_box;

/* Generated content wraps the real children: a ::before box leads and an
 * ::after box trails. Out of line, so the boxes it builds take no room in
 * each frame of the recursive tree build. */
#[inline(never)]
pub(super) fn add_pseudos(w: &mut Walk, id: usize, link: &Option<String>, kids: &mut Vec<BoxNode>) {
    let host = &w.styles[id];
    let blockify = host.is_flex || host.is_grid;
    for p in &w.pseudos[id] {
        let (Some(text), first) = (&p.text, p.kind == PseudoText::BEFORE) else { continue };
        if !matches!(p.kind, PseudoText::BEFORE | PseudoText::AFTER) {
            continue;
        }
        let Some(node) = pseudo_box(p, text, link, (id, blockify)) else { continue };
        *w.count += 1;
        if first {
            kids.insert(0, node);
        } else {
            kids.push(node);
        }
    }
}
