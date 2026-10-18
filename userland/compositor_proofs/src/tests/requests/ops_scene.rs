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

//! Pointer, damage, capture and scene requests.

use super::ops::{b, some, Op};
use crate::requests::{area, capture, place, pointer};

fn wide(lo: u32, hi: u32) -> u64 {
    lo as u64 | (hi as u64) << 32
}

fn cursor(x: &[u8]) -> Option<Vec<u64>> {
    pointer::cursor_update(x).map(|c| some(&[c.x, c.y, b(c.visible)]))?
}

fn damage(x: &[u8]) -> Option<Vec<u64>> {
    area::damage_commit(x).map(|a| some(&[a.x, a.y, a.width, a.height]))?
}

fn submit(x: &[u8]) -> Option<Vec<u64>> {
    let p = place::scene_submit(x)?;
    Some([p.handle, p.x.into(), p.y.into(), p.width.into(), p.height.into(), p.z.into()].into())
}

fn submitted(w: &[u32]) -> Option<Vec<u64>> {
    (w[7] == 0).then(|| Some([vec![wide(w[0], w[1])], some(&w[2..7])?].concat()))?
}

fn focus(x: &[u8]) -> Option<Vec<u64>> {
    some(&[pointer::focus_set(x)?])
}

fn removed(x: &[u8]) -> Option<Vec<u64>> {
    place::scene_remove(x).map(|r| some(&[r.z, b(r.band_only)]))?
}

pub fn pointer_and_scene() -> Vec<Op> {
    vec![
        ("focus_set", 8, &[7, 0], focus, |w| (w[1] == 0).then(|| some(&w[..1]))?),
        ("cursor_update", 16, &[10, 20, 1, 0], cursor, |w| {
            (w[3] == 0).then(|| some(&[w[0], w[1], b(w[2] != 0)]))?
        }),
        ("damage_commit", 16, &[1, 2, 3, 4], damage, |w| some(&w[..4])),
        (
            "screenshot",
            8,
            &[5, 0],
            |x| Some(vec![capture::screenshot(x)?]),
            |w| Some(vec![wide(w[0], w[1])]),
        ),
        (
            "accessibility_get",
            4,
            &[0],
            |x| some(&[capture::accessibility_get(x)? as u32]),
            |w| some(&w[..1]),
        ),
        ("scene_submit", 32, &[9, 0, 1, 2, 3, 4, 5, 0], submit, submitted),
        ("scene_remove", 8, &[3, 1], removed, |w| some(&[w[0], w[1] & 1])),
    ]
}
