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

use super::affine::Affine;
use super::aspect::fit;
use super::attr::attr;
use super::defs::ref_id;
use super::num::{num_list, parse_len};
use super::state::Paint;
use super::walk::Walk;
use super::xml::next_tag;

/// How deeply `use` references may nest.
const MAX_DEPTH: u32 = 8;

impl Walk<'_, '_> {
    /// Paint the element a `use` names, moved by its x and y; a symbol
    /// with a viewBox is fitted into the use's width and height.
    pub(super) fn use_ref(&mut self, attrs: &str, p: Paint) {
        let target = attr(attrs, "href").and_then(ref_id).and_then(|id| self.defs.element(id));
        let Some(at) = target.filter(|_| self.depth < MAX_DEPTH) else { return };
        let len = |n: &str| attr(attrs, n).and_then(parse_len);
        let mut t = p.t.then(&Affine::translate(len("x").unwrap_or(0.0), len("y").unwrap_or(0.0)));
        if let Some((sym, _)) = next_tag(self.defs.doc, at).filter(|(s, _)| s.name == "symbol") {
            let vb = attr(sym.attrs, "viewBox")
                .map(num_list)
                .filter(|v| v.len() == 4 && v[2] > 0.0 && v[3] > 0.0);
            if let (Some(v), Some(w), Some(h)) = (vb, len("width"), len("height")) {
                let par = attr(sym.attrs, "preserveAspectRatio");
                t = t.then(&fit(par, [v[0], v[1], v[2], v[3]], w, h));
            }
        }
        self.depth += 1;
        self.run(at, Paint { t, ..p }, true);
        self.depth -= 1;
    }
}
