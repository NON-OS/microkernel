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

use std::sync::{Mutex, MutexGuard, PoisonError};

use capsule_browser_proofs::browser::pages::body::draw;
use capsule_browser_proofs::browser::shell::Shell;
use capsule_browser_proofs::browser::ui::list_geom::ListGeom;
use capsule_browser_proofs::browser::ui::page_part::PagePart;
use capsule_browser_proofs::browser::ui::rect::Rect;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

static SCALE: Mutex<()> = Mutex::new(());
pub const W: i32 = 2400;
pub const H: i32 = 1500;

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

pub fn geom(s: &Shell) -> ListGeom {
    ListGeom::new(W, H, s.page_scroll)
}

/// The pixels the page draws, its controls laid out in `c`.
pub fn paint(s: &Shell, c: &mut Controls<PagePart>) -> Vec<u32> {
    let (w, h) = (W as u32, H as u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    draw(s, &geom(s), 3_600_000, &mut fb, c);
    px
}

/// Every control that acts somewhere, with the box around where it acts.
pub fn spots(c: &Controls<PagePart>) -> Vec<(PagePart, Rect)> {
    let mut out: Vec<(PagePart, (i32, i32, i32, i32))> = Vec::new();
    for y in 0..H {
        for x in 0..W {
            let Some(p) = c.at(x, y) else { continue };
            match out.iter_mut().find(|(k, _)| *k == p) {
                Some((_, b)) => *b = (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y)),
                None => out.push((p, (x, y, x, y))),
            }
        }
    }
    out.into_iter().map(|(p, b)| (p, Rect::new(b.0, b.1, b.2 - b.0 + 1, b.3 - b.1 + 1))).collect()
}

pub fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    (0..a.len()).filter(|&n| a[n] != b[n]).map(|n| (n as i32 % W, n as i32 / W)).collect()
}

pub fn mid(r: Rect) -> (i32, i32) {
    (r.x + r.w / 2, r.y + r.h / 2)
}
