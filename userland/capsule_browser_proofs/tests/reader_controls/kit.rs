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

use capsule_browser_proofs::browser::reader_card::panel::paint as lay;
use capsule_browser_proofs::browser::shell::kept::ReaderPrefs;
use capsule_browser_proofs::browser::ui::reader_geom::{panel, ReaderGeom};
use capsule_browser_proofs::browser::ui::reader_hit::Control;
use capsule_browser_proofs::browser::ui::rect::Rect;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

static SCALE: Mutex<()> = Mutex::new(());
pub const W: i32 = 1400;
pub const H: i32 = 1200;

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

/// Settings with room for the text size to go either way.
pub fn prefs() -> ReaderPrefs {
    ReaderPrefs { theme: 0, size: 2, width: 1, show_time: false }
}

/// The card in a content area `W` wide.
pub fn geom() -> ReaderGeom {
    panel(W)
}

/// Every control of the card, with where it is drawn.
pub fn controls(g: &ReaderGeom) -> Vec<(Control, Rect)> {
    let mut v = vec![(Control::Close, g.close), (Control::Smaller, g.minus)];
    v.extend([(Control::Larger, g.plus), (Control::Time, g.time)]);
    v.extend((0..3).map(|i| (Control::Theme(i as u8), g.swatches[i])));
    v.extend((0..3).map(|i| (Control::Width(i as u8), g.widths[i])));
    v
}

/// The pixels the card draws in a content area `w` by `H`.
pub fn paint(w: i32, p: &ReaderPrefs, c: &mut Controls<Control>) -> Vec<u32> {
    let (wu, h) = (w.max(1) as u32, H as u32);
    let mut px = vec![0u32; (wu * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: wu, width: wu, height: h };
    lay(p, &panel(w), &mut fb, c);
    px
}

pub fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    (0..a.len()).filter(|&n| a[n] != b[n]).map(|n| (n as i32 % W, n as i32 / W)).collect()
}

pub fn mid(r: Rect) -> (i32, i32) {
    (r.x + r.w / 2, r.y + r.h / 2)
}
