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

use capsule_browser_proofs::browser::tabbar::strip::{self, Shown};
use capsule_browser_proofs::browser::ui::tabs_geom::{Strip, TabPart};
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

pub const W: i32 = 900;
static SCALE: Mutex<()> = Mutex::new(());

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

/// `n` tabs with `active` chosen, in a strip as tall as the titlebar at `q`.
pub fn strip(n: usize, active: usize, q: u32) -> Strip {
    Strip { n, active, w: W, h: 10 * q as i32 }
}

/// The pixels the strip draws, every tab named and led by an icon.
pub fn paint(s: &Strip, busy: &[bool], c: &mut Controls<TabPart>) -> Vec<u32> {
    let labels: Vec<String> = (0..s.n).map(|i| format!("Page {i}")).collect();
    let icons: Vec<IconId> = (0..s.n).map(|i| [IconId::Home, IconId::Globe][i % 2]).collect();
    let (w, h) = (s.w as u32, s.h as u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    strip::paint(&mut fb, s, &Shown { labels: &labels, icons: &icons, busy }, c);
    px
}

pub fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as i32 % W), (n as i32 / W));
    (0..a.len()).filter(|&n| a[n] != b[n]).map(at).collect()
}

/// The points of the strip `c` answers with a part `keep` accepts.
pub fn spots(c: &Controls<TabPart>, s: &Strip, keep: impl Fn(TabPart) -> bool) -> Vec<(i32, i32)> {
    let all = (0..s.h).flat_map(|y| (0..s.w).map(move |x| (x, y)));
    all.filter(|&(x, y)| c.at(x, y).is_some_and(&keep)).collect()
}

/// The centre of the box `part` takes presses in.
pub fn centre(c: &Controls<TabPart>, s: &Strip, part: TabPart) -> (i32, i32) {
    let p = spots(c, s, |a| a == part);
    let (x0, x1) = (p.iter().map(|p| p.0).min().unwrap(), p.iter().map(|p| p.0).max().unwrap());
    let (y0, y1) = (p.iter().map(|p| p.1).min().unwrap(), p.iter().map(|p| p.1).max().unwrap());
    ((x0 + x1) / 2, (y0 + y1) / 2)
}

/// Whether `part` belongs to tab `i`: the tab itself or its close mark.
pub fn of(i: usize) -> impl Fn(TabPart) -> bool {
    move |a| a == TabPart::Tab(i) || a == TabPart::Close(i)
}
