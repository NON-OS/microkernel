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

use alloc::vec::Vec;

use crate::browser::css::calc::split_top::{items, words};
use crate::browser::css::calc::{eval_value, V};
use crate::browser::css::computed::Computed;
use crate::browser::layout::boxmodel::Rel;

/* Which axis a position word names: left/right, top/bottom, or either. */
#[derive(Clone, Copy, PartialEq)]
enum Axis {
    X,
    Y,
    Any,
}

/// background-position from its words: one or two keywords, lengths or
/// percentages (a lone value centres the other axis, and a vertical
/// keyword first is swapped into place), or edge-offset pairs such as
/// "right 10px bottom 20px". None when the words are not a position.
pub(in crate::browser::css) fn bg_pos(ws: &[&str], fs: u32) -> Option<[Rel; 2]> {
    let t: Vec<(Axis, Rel)> = ws.iter().map(|w| tok(w, fs)).collect::<Option<_>>()?;
    let center = (0, 500);
    match t.as_slice() {
        [(Axis::Y, r)] => Some([center, *r]),
        [(_, r)] => Some([*r, center]),
        [(ka, a), (kb, b)] if *ka == Axis::Y || *kb == Axis::X => Some([*b, *a]),
        [(_, a), (_, b)] => Some([*a, *b]),
        [(ka, ea), (_, oa), (kb, eb), (_, ob)] if *ka != *kb || *ka == Axis::Any => {
            let at = |e: Rel, o: Rel| if e.1 == 1000 { (-o.0, 1000 - o.1) } else { o };
            let (p, q) = (at(*ea, *oa), at(*eb, *ob));
            Some(if *ka == Axis::Y { [q, p] } else { [p, q] })
        }
        _ => None,
    }
}

fn tok(w: &str, fs: u32) -> Option<(Axis, Rel)> {
    Some(match w.to_ascii_lowercase().as_str() {
        "left" => (Axis::X, (0, 0)),
        "right" => (Axis::X, (0, 1000)),
        "top" => (Axis::Y, (0, 0)),
        "bottom" => (Axis::Y, (0, 1000)),
        "center" => (Axis::Any, (0, 500)),
        _ => match eval_value(w, fs as f32)? {
            V::Num(0.0) => (Axis::Any, (0, 0)),
            V::Len { px, pml } => (Axis::Any, (px as i32, pml as i32)),
            _ => return None,
        },
    })
}

/// The first layer of a background-position list.
pub(in crate::browser::css) fn apply_bg_pos(c: &mut Computed, value: &str, fs: u32) {
    let ws: Vec<&str> = words(items(value).next().unwrap_or("")).collect();
    if let Some(p) = bg_pos(&ws, fs) {
        c.bg_layer.pos = p;
    }
}
