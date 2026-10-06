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

/// A 2D affine map, x' = a x + c y + e and y' = b x + d y + f, as CSS
/// matrix(a, b, c, d, e, f). The translation parts are [px, per px of box
/// width, per px of box height], so translate(-50%) composes exactly before
/// the box size is known.
#[derive(Clone, Copy, PartialEq)]
pub struct Affine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: [f32; 3],
    pub f: [f32; 3],
}

impl Affine {
    pub const IDENTITY: Affine =
        Affine { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: [0.0; 3], f: [0.0; 3] };

    /// A translation by box-relative lengths: x per mille of the width, y
    /// per mille of the height.
    pub fn translate(x: (i32, i32), y: (i32, i32)) -> Affine {
        let (e, f) =
            ([x.0 as f32, x.1 as f32 / 1000.0, 0.0], [y.0 as f32, 0.0, y.1 as f32 / 1000.0]);
        Affine { e, f, ..Affine::IDENTITY }
    }

    /// A linear map with no translation.
    pub fn linear(a: f32, b: f32, c: f32, d: f32) -> Affine {
        Affine { a, b, c, d, ..Affine::IDENTITY }
    }

    /// self then `n` in CSS list order: the result maps p to self(n(p)).
    pub fn then(self, n: Affine) -> Affine {
        let lin = |x: &[f32; 3], y: &[f32; 3], t: &[f32; 3], k1: f32, k2: f32| -> [f32; 3] {
            core::array::from_fn(|i| k1 * x[i] + k2 * y[i] + t[i])
        };
        Affine {
            a: self.a * n.a + self.c * n.b,
            b: self.b * n.a + self.d * n.b,
            c: self.a * n.c + self.c * n.d,
            d: self.b * n.c + self.d * n.d,
            e: lin(&n.e, &n.f, &self.e, self.a, self.c),
            f: lin(&n.e, &n.f, &self.f, self.b, self.d),
        }
    }

    /// The translation in px for a border box of `w` x `h`.
    pub fn offset(&self, w: f32, h: f32) -> (f32, f32) {
        (self.e[0] + self.e[1] * w + self.e[2] * h, self.f[0] + self.f[1] * w + self.f[2] * h)
    }

    /// Map (x, y), relative to the transform origin, for a `w` x `h` box.
    pub fn apply(&self, x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
        let (e, f) = self.offset(w, h);
        (self.a * x + self.c * y + e, self.b * x + self.d * y + f)
    }
}
