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

use capsule_browser_proofs::browser::settings::controls::paint as lay;
use capsule_browser_proofs::browser::settings::look::Shows;
use capsule_browser_proofs::browser::ui::rect::Rect;
use capsule_browser_proofs::browser::ui::settings_geom::{Control, Item, Kind, SettingsGeom};
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

static SCALE: Mutex<()> = Mutex::new(());
pub const W: i32 = 1100;
pub const H: i32 = 2600;

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

/// A page with something behind every button: a proxy, history, a saved
/// download, two bookmarks and a text size with room both ways.
pub fn live() -> Shows {
    let had = Shows { size: 1, network: Some(0), proxy: true, history: true, ..Shows::default() };
    Shows { downloads: true, bookmarks: 2, ..had }
}

/// The page `W` wide, tall enough that nothing scrolls.
pub fn geom() -> SettingsGeom {
    SettingsGeom::new(W, H, 0)
}

/// Every control the page places, with where it is drawn.
pub fn controls(g: &SettingsGeom) -> Vec<(Control, Rect)> {
    let ctl = |it: &Item| match it.kind {
        Kind::Ctl(c) => Some((c, it.rect)),
        _ => None,
    };
    g.items.iter().filter_map(ctl).collect()
}

/// The pixels the buttons draw over a page `W` by `H`.
pub fn paint(g: &SettingsGeom, s: &Shows, c: &mut Controls<Control>) -> Vec<u32> {
    let (w, h) = (W as u32, H as u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    lay(&mut fb, g, H, s, c);
    px
}

pub fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    (0..a.len()).filter(|&n| a[n] != b[n]).map(|n| (n as i32 % W, n as i32 / W)).collect()
}

pub fn mid(r: Rect) -> (i32, i32) {
    (r.x + r.w / 2, r.y + r.h / 2)
}
