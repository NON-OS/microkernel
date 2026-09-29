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
use alloc::vec::Vec;

use crate::browser::css::PseudoText;

use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::Walk;

/* Generated content wraps the real children: a ::before box leads and a
 * ::after box trails, each a text leaf styled by its own cascade. */
pub(super) fn add_pseudos(w: &mut Walk, id: usize, link: &Option<String>, kids: &mut Vec<BoxNode>) {
    let Some((before, after)) = w.pseudos.get(id) else {
        return;
    };
    if let Some(b) = before {
        *w.count += 1;
        kids.insert(0, pseudo(b, link, id));
    }
    if let Some(a) = after {
        *w.count += 1;
        kids.push(pseudo(a, link, id));
    }
}

fn pseudo(p: &PseudoText, link: &Option<String>, id: usize) -> BoxNode {
    let mut node = leaf(BoxKind::Text(p.text.clone()), &p.style, link, id);
    node.style = p.style;
    node
}
