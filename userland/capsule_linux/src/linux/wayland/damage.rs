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

//! What a client said changed on its surface since its last commit, in
//! buffer pixels, and that part laid into the window. Pure, so the host
//! proofs hold it.

/// Nothing said, the box from (x0, y0) up to (x1, y1), or all of it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Damage {
    #[default]
    None,
    Rect(u32, u32, u32, u32),
    Whole,
}

impl Damage {
    /// With the box at (`x`, `y`), `w` by `h`, `scale` buffer pixels to
    /// one it was named in, added. Clients name "everything" with
    /// i32::MAX sizes, which stay in range.
    pub fn add(self, (x, y, w, h): (i32, i32, i32, i32), scale: u32) -> Damage {
        let s = i64::from(scale.max(1));
        let px = |v: i32| (i64::from(v) * s).clamp(0, i64::from(u32::MAX)) as u32;
        let (x0, y0) = (px(x), px(y));
        let (x1, y1) = (px(x.saturating_add(w)), px(y.saturating_add(h)));
        if w <= 0 || h <= 0 || x1 <= x0 || y1 <= y0 {
            return self;
        }
        match self {
            Damage::Whole => Damage::Whole,
            Damage::None => Damage::Rect(x0, y0, x1, y1),
            Damage::Rect(a, b, c, d) => Damage::Rect(a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
        }
    }

    /// The part on a `w` by `h` buffer: Whole when that is all of it.
    pub fn on(self, w: u32, h: u32) -> Damage {
        let Damage::Rect(x0, y0, x1, y1) = self else { return self };
        let (x1, y1) = (x1.min(w), y1.min(h));
        match () {
            _ if x0 >= x1 || y0 >= y1 => Damage::None,
            _ if (x0, y0, x1, y1) == (0, 0, w, h) => Damage::Whole,
            _ => Damage::Rect(x0, y0, x1, y1),
        }
    }
}

/// What a commit shows: what was said, or all of a buffer newly attached
/// with nothing said, as a client that names no damage means it all.
pub fn at_commit(said: Damage, attached: bool) -> Damage {
    if said == Damage::None && attached {
        return Damage::Whole;
    }
    said
}
