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

//! What a shell prompt paints and where its clicks land, for the prompt
//! proofs: the box a click on a button names, and what a pointer lights.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Dialog, Pointer, Rect};

use crate::shell_layer::layer;
use crate::shell_prompt_view::draw;

pub(crate) const LAYER: Rect = Rect { x: 0, y: 0, w: 960, h: 540 };

pub(crate) fn inside(r: Rect) -> impl Iterator<Item = (i32, i32)> {
    (r.y..r.y + r.h).flat_map(move |y| (r.x..r.x + r.w).map(move |x| (x as i32, y as i32)))
}

pub(crate) fn painted(d: Dialog, p: &Pointer) -> Vec<u32> {
    let (w, h) = (LAYER.w, LAYER.h);
    let mut words = vec![roles().bg.argb(); (w * h) as usize];
    draw(&mut unsafe { layer(words.as_mut_ptr() as u64, w * 4, w, h) }, d, LAYER, p);
    words
}

fn edge(mut inn: i32, mut out: i32, on: impl Fn(i32) -> bool) -> i32 {
    while (out - inn).abs() > 1 {
        let m = (inn + out) / 2;
        *(if on(m) { &mut inn } else { &mut out }) = m;
    }
    inn
}

/// The box a click on `b` names: from its focus ring out by halving, as hits are rects.
pub(crate) fn hit_box(d: &Dialog, b: usize) -> Rect {
    let none = &Pointer::new();
    let (plain, rung) = (painted(d.focus(usize::MAX), none), painted(d.focus(b), none));
    let ring = inside(LAYER).zip(plain.iter().zip(&rung)).filter(|(_, (p, r))| p != r);
    let ring: Vec<_> = ring.map(|(at, _)| at).collect();
    let span = |f: fn(&(i32, i32)) -> i32| (ring.iter().map(f).min(), ring.iter().map(f).max());
    let ((x0, x1), (y0, y1)) = (span(|p| p.0), span(|p| p.1));
    let (cx, cy) = ((x0.unwrap() + x1.unwrap()) / 2, (y0.unwrap() + y1.unwrap()) / 2);
    let on = |x: i32, y: i32| d.hit(LAYER, x, y) == Some(b);
    assert!(on(cx, cy), "button {b} is not hit inside its ring");
    let (p, across, down) = (d.panel(LAYER), |x| on(x, cy), |y| on(cx, y));
    let (l, r) = (edge(cx, p.x as i32 - 1, across), edge(cx, (p.x + p.w) as i32, across));
    let (t, u) = (edge(cy, p.y as i32 - 1, down), edge(cy, (p.y + p.h) as i32, down));
    Rect { x: l as u32, y: t as u32, w: (r - l + 1) as u32, h: (u - t + 1) as u32 }
}

pub(crate) fn holds(r: Rect, (x, y): (i32, i32)) -> bool {
    let (x0, y0) = (r.x as i32, r.y as i32);
    x >= x0 && y >= y0 && x < x0 + r.w as i32 && y < y0 + r.h as i32
}

/// Whether drawing with `p` changes some pixel, and only pixels in `r`.
pub(crate) fn drawn_in(d: Dialog, p: &Pointer, r: Rect) -> bool {
    let (plain, lit) = (painted(d, &Pointer::new()), painted(d, p));
    let mut moved = inside(LAYER).zip(plain.iter().zip(&lit)).filter(|(_, (a, b))| a != b);
    let mut moved = moved.by_ref().peekable();
    moved.peek().is_some() && moved.all(|(at, _)| holds(r, at))
}
