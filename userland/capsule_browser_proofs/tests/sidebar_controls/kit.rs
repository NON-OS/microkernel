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

use capsule_browser_proofs::browser::sidebar::nav::{self, Look};
use capsule_browser_proofs::browser::ui::layout::{compute, SidebarMode};
use capsule_browser_proofs::browser::ui::rect::Rect;
use capsule_browser_proofs::browser::ui::sidebar_geom::RowKind;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

static SCALE: Mutex<()> = Mutex::new(());

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

/// The sidebar of a 1400 px wide window `h` tall, folded to a rail or not.
pub fn side(rail: bool, h: i32) -> Rect {
    let mode = if rail { SidebarMode::Rail } else { SidebarMode::Full };
    compute(1400, h, mode).sidebar
}

/// Two groups, three tabs, Tabs on show, room for more groups.
pub fn look(on: Option<RowKind>, full: bool) -> Look<'static> {
    Look { on, tabs: 3, groups: &[("Work", 2), ("Play", 1)], full }
}

/// The pixels the rows draw, in a buffer reaching the sidebar's far corner.
pub fn paint(rail: bool, s: Rect, k: &Look, c: &mut Controls<RowKind>) -> Vec<u32> {
    let (w, h) = (s.right() as u32, s.bottom() as u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    nav::paint(&mut fb, rail, s, k, c);
    px
}

pub fn changed(a: &[u32], b: &[u32], s: Rect) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as i32 % s.right()), (n as i32 / s.right()));
    (0..a.len()).filter(|&n| a[n] != b[n]).map(at).collect()
}

pub fn mid(r: Rect) -> (i32, i32) {
    (r.x + r.w / 2, r.y + r.h / 2)
}
