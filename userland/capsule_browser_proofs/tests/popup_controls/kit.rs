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

use capsule_browser_proofs::browser::dom;
use capsule_browser_proofs::browser::event::select_list::SelectList;
use capsule_browser_proofs::browser::ui::select_menu::{draw, open_at, place};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Rect;

static SCALE: Mutex<()> = Mutex::new(());
pub const W: u32 = 1000;
pub const H: u32 = 800;
pub const PAGE: Rect = Rect { x: 0, y: 0, w: W, h: H };

/// Five options, the second shown, the third disabled, two under a heading.
pub const SIZES: &[u8] = b"<select name=s><option>Small<option selected value=m>Medium\
    <option disabled>Gone<optgroup label=Big><option>Large<option>Huge</optgroup></select>";

/// Holds the display scale at `q` quarters for the rest of a test.
pub fn at_scale(q: u32) -> MutexGuard<'static, ()> {
    let held = SCALE.lock().unwrap_or_else(PoisonError::into_inner);
    set_scale(q);
    held
}

pub fn px(logical: u32) -> u32 {
    scale().px(logical)
}

/// The list of the select in `html` drawn at `select`, and where it opens.
pub fn open(html: &[u8], select: Rect, bounds: Rect) -> (SelectList, Rect) {
    let d = dom::parse(html);
    let id = d.nodes.iter().position(|n| n.tag == "select").expect("a select");
    let mut list = SelectList::open(&d, id).expect("a list");
    open_at(&mut list, select, bounds);
    let at = place(&list, select, bounds);
    (list, at)
}

/// The page `W` by `H` once `f` has drawn on it.
pub fn paint(f: impl FnOnce(&mut PaintBuffer)) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    f(&mut PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H });
    px
}

pub fn list_px(list: &SelectList, at: Rect) -> Vec<u32> {
    paint(|fb| draw(list, fb, at))
}

pub fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n % W as usize) as i32, (n / W as usize) as i32);
    (0..a.len()).filter(|&n| a[n] != b[n]).map(at).collect()
}

pub fn inside(r: Rect, (x, y): (i32, i32)) -> bool {
    let (x, y, r) = (x as i64, y as i64, [r.x, r.y, r.w, r.h].map(i64::from));
    x >= r[0] && y >= r[1] && x < r[0] + r[2] && y < r[1] + r[3]
}
