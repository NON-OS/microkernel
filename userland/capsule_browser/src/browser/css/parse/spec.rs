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

/* Specificity as Selectors 4 counts it: ids; classes, attributes and
 * pseudo-classes; types and pseudo-elements. Packed with ten bits per level
 * and each level clamped, a lower level can never carry into a higher one,
 * so packed values compare exactly like the triples. */
#[derive(Clone, Copy, Default)]
pub(super) struct Spec {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl Spec {
    pub fn add(self, o: Spec) -> Spec {
        Spec {
            a: self.a.saturating_add(o.a),
            b: self.b.saturating_add(o.b),
            c: self.c.saturating_add(o.c),
        }
    }

    pub fn pack(self) -> u32 {
        (self.a.min(0x3FF) << 20) | (self.b.min(0x3FF) << 10) | self.c.min(0x3FF)
    }

    pub fn unpack(v: u32) -> Spec {
        Spec { a: (v >> 20) & 0x3FF, b: (v >> 10) & 0x3FF, c: v & 0x3FF }
    }
}
