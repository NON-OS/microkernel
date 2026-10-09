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

use crate::browser::css::decl::Decl;
use crate::browser::css::rule::Rule;

/* A rule matched by an element (or one of its pseudo-elements), sorted
 * by (elem, layer, spec, rule): cascade order within one origin. */
#[derive(Clone, Copy)]
pub(in crate::browser::css) struct Hit {
    pub elem: u8,
    pub layer: u16,
    pub spec: u32,
    pub rule: u32,
}

/* The declarations that apply to one box, by origin. */
pub(in crate::browser::css) struct Order<'a> {
    pub ua: (&'a [Rule], &'a [Hit]),
    /* Presentational hints: author level, specificity zero, first. */
    pub hints: &'a [Decl],
    pub author: (&'a [Rule], &'a [Hit]),
    /* The style attribute. */
    pub inline: &'a [Decl],
}

impl<'a> Order<'a> {
    /* Every declaration in cascade order, lowest precedence first: UA,
     * hints, author and inline normal ones, then author important ones
     * with the layer order reversed, inline important, UA important.
     * With `need` set, only rules sharing a flag bit with it are read. */
    pub fn each(&self, need: u16, f: &mut dyn FnMut(&'a Decl)) {
        let (ur, uh) = self.ua;
        let (ar, ah) = self.author;
        rules(ur, uh, need, false, f);
        self.hints.iter().for_each(&mut *f);
        rules(ar, ah, need, false, f);
        self.inline.iter().filter(|d| !d.important).for_each(&mut *f);
        let mut end = ah.len();
        while end > 0 {
            let layer = ah[end - 1].layer;
            let start = ah[..end].iter().rposition(|h| h.layer != layer).map_or(0, |p| p + 1);
            rules(ar, &ah[start..end], need, true, f);
            end = start;
        }
        self.inline.iter().filter(|d| d.important).for_each(&mut *f);
        rules(ur, uh, need, true, f);
    }
}

/* The declarations of the hit rules of one importance. An important
 * pass reads only rules holding an important declaration. */
fn rules<'a>(rs: &'a [Rule], hits: &[Hit], need: u16, imp: bool, f: &mut dyn FnMut(&'a Decl)) {
    for h in hits {
        let Some(r) = rs.get(h.rule as usize) else { continue };
        if (imp && r.flags & Rule::IMPORTANT == 0) || (need != 0 && r.flags & need == 0) {
            continue;
        }
        r.decls.iter().filter(|d| d.important == imp).for_each(&mut *f);
    }
}
