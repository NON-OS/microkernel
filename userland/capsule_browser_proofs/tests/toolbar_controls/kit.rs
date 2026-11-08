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

use capsule_browser_proofs::browser::shell::route::Route;
use capsule_browser_proofs::browser::toolbar::{badge, buttons, look::Shows};
use capsule_browser_proofs::browser::ui::limits::TOOLBAR_H;
use capsule_browser_proofs::browser::ui::rect::Rect;
use capsule_browser_proofs::browser::ui::toolbar_geom::{self, Part, ToolbarGeom};
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

const W: i32 = 1200;
static SCALE: Mutex<()> = Mutex::new(());

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

/// A toolbar with every control live, over the mixnet.
pub fn live() -> Shows {
    let (back, fwd, loading, page, marked, ready, reading) =
        (true, true, false, true, false, true, false);
    Shows { back, fwd, loading, page, marked, ready, reading, route: Route::Mixnet }
}

pub fn geom() -> ToolbarGeom {
    toolbar_geom::buttons(Rect::new(0, 0, W, TOOLBAR_H))
}

/// The pixels a toolbar paint draws the controls with, the badge drawn over
/// the field as the field's painter draws it.
pub fn paint(s: &Shows, c: &mut Controls<Part>) -> Vec<u32> {
    let (g, (w, h)) = (geom(), (W as u32, TOOLBAR_H as u32));
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    buttons::paint(&mut fb, &g, s, c);
    badge::paint(&mut fb, g.route, s.route, c.state(Part::Route));
    px
}

pub fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as i32 % W), (n as i32 / W));
    (0..a.len()).filter(|&n| a[n] != b[n]).map(at).collect()
}
