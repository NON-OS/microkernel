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
use crate::requests::{area, capture, place, place_keyed, pointer};

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

fn submit_layer(x: &[u8]) -> Option<Vec<u64>> {
    let k = place_keyed::scene_submit_layer(x)?;
    let p = k.placed;
    let r = k.replaces.unwrap_or(0);
    let words = [p.x, p.y, p.width, p.height, p.z, k.window_id].map(u64::from);
    Some([vec![p.handle], words.to_vec(), vec![r]].concat())
}

// A keyed submit: a handle and a window id that are not 0, flags and the
// reserved word 0; a replaced handle of 0 is none.
fn submitted_layer(w: &[u32]) -> Option<Vec<u64>> {
    let w = w.get(..12)?;
    let ok = wide(w[0], w[1]) != 0 && w[7] != 0 && w[10] == 0 && w[11] == 0;
    ok.then(|| Some([vec![wide(w[0], w[1])], some(&w[2..8])?, vec![wide(w[8], w[9])]].concat()))?
}

fn removed_layer(x: &[u8]) -> Option<Vec<u64>> {
    place_keyed::scene_remove_layer(x).map(|r| vec![r.handle, r.z.into()])
}

fn focus_window(x: &[u8]) -> Option<Vec<u64>> {
    place_keyed::focus_set_window(x).map(|(p, w)| some(&[p, w]))?
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
        (
            "scene_submit_layer",
            48,
            &[9, 0, 1, 2, 3, 4, 2, 5, 8, 0, 0, 0],
            submit_layer,
            submitted_layer,
        ),
        ("scene_remove_layer", 16, &[9, 0, 2, 0], removed_layer, |w| {
            let h = wide(w[0], w[1]);
            (h != 0 && w[3] == 0).then(|| vec![h, w[2].into()])
        }),
        ("focus_set_window", 8, &[7, 3], focus_window, |w| some(&w[..2])),
    ]
}
