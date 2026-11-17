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

//! The shared widgets on a shell layer: a widget changes only pixels inside
//! its box and the layer, and a translucent fill blends, at any scale.

use nonos_toolkit::paint::{mixer::over, PaintBuffer};
use nonos_toolkit::theme::{derive::opaque, roles};
use nonos_toolkit::widgets::{draw, Align, Label, Rect, Separator};

use crate::shell_layer::layer;

/// The layer's size, and its row in words with spare words past the width.
const W: u32 = 160;
const H: u32 = 120;
pub(crate) const STRIDE: u32 = 176;
/// The name the labels draw.
pub(crate) const NAME: &str = "Files";

/// A layer in the theme's ground with a row of guard words past its end,
/// painted on through the shell's own view of a layer.
pub(crate) fn painted(paint: impl FnOnce(&mut PaintBuffer)) -> Vec<u32> {
    let mut words = vec![roles().bg.argb(); (STRIDE * (H + 1)) as usize];
    paint(&mut unsafe { layer(words.as_mut_ptr() as u64, STRIDE * 4, W, H) });
    words
}

/// Each word that is no longer the ground, as its column and row.
pub(crate) fn changed(words: &[u32]) -> Vec<(u32, u32)> {
    let moved = |&i: &u32| words[i as usize] != roles().bg.argb();
    (0..words.len() as u32).filter(moved).map(|i| (i % STRIDE, i / STRIDE)).collect()
}

#[test]
fn a_widget_paints_only_inside_its_box_and_the_layer() {
    let (inner, edge) =
        (Rect { x: 20, y: 10, w: 100, h: 70 }, Rect { x: 120, y: 100, w: 80, h: 40 });
    let marks = changed(&painted(|fb| {
        Label::new(NAME).align(Align::Centre).draw(fb, inner);
        Separator::new().draw(fb, edge);
    }));
    let drew = |r: Rect| marks.iter().any(|&(x, y)| r.contains(x, y));
    assert!(drew(inner) && drew(edge), "a widget drew nothing");
    let kept =
        |&(x, y): &(u32, u32)| x < W && y < H && (inner.contains(x, y) || edge.contains(x, y));
    assert!(marks.iter().all(kept), "a pixel changed off the box or the layer");
}

#[test]
fn a_translucent_fill_blends_over_the_layer() {
    let r = roles();
    let words = painted(|fb| draw::blend(fb, Rect { x: 0, y: 0, w: W, h: H }, r.scrim));
    let want = over(r.bg.argb(), r.scrim.argb());
    assert!(want != r.bg.argb() && want != opaque(r.scrim.argb()), "the scrim did not blend");
    let marks = changed(&words);
    assert_eq!(marks.len(), (W * H) as usize, "the scrim missed some of the layer");
    assert!(marks.iter().all(|&(x, y)| x < W && y < H && words[(y * STRIDE + x) as usize] == want));
}
