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

use capsule_browser_proofs::browser::ui::rect::Rect;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;

static SCALE: Mutex<()> = Mutex::new(());
pub const W: i32 = 2000;
pub const H: i32 = 2600;

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

/// The pixels `draw` paints over a blank `W` by `H` page.
pub fn paint(draw: impl FnOnce(&mut PaintBuffer)) -> Vec<u32> {
    let (w, h) = (W as u32, H as u32);
    let mut px = vec![0u32; (w * h) as usize];
    draw(&mut PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h });
    px
}

/// The box around every pixel that differs between `a` and `b`.
pub fn ink_box(a: &[u32], b: &[u32]) -> Option<Rect> {
    let at = |n: usize| (n as i32 % W, n as i32 / W);
    let hit: Vec<(i32, i32)> = (0..a.len()).filter(|&n| a[n] != b[n]).map(at).collect();
    let (x0, y0) = (hit.iter().map(|p| p.0).min()?, hit.iter().map(|p| p.1).min()?);
    let (x1, y1) = (hit.iter().map(|p| p.0).max()?, hit.iter().map(|p| p.1).max()?);
    Some(Rect::new(x0, y0, x1 - x0 + 1, y1 - y0 + 1))
}

pub fn inside(r: Rect, o: Rect) -> bool {
    r.x >= o.x && r.y >= o.y && r.right() <= o.right() && r.bottom() <= o.bottom()
}
