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
//! What a death proof holds the driver to: the frames it holds at that
//! moment, the planes on them only, and the steps worth going wrong at.

use super::kit::{rig, Model, Wrong};
use crate::model::frames::CURSOR_IOVA;
use crate::tests::confine::kit::confined;
use crate::tests::wire_rig::{h0, Rig};

/// The IOVA ranges the driver holds: each surface of its pair and of the
/// pair a mode set replaced if it has not settled, and the cursor image.
pub fn held(d: &Rig) -> Vec<(u64, u64)> {
    let f = &h0(d).frames;
    let pair = |(i, l): ([u64; 2], usize)| i.map(|a| (a, a + l as u64));
    let mut v = vec![(CURSOR_IOVA, CURSOR_IOVA + 16384)];
    v.extend(pair((f.iova, f.len)));
    v.extend(f.was.into_iter().flat_map(pair));
    v
}

/// The steps worth going wrong at in a clean run of `work` on a fresh rig:
/// every register write, and 24 of the GGTT writes between, evenly spread.
pub fn steps(work: impl FnOnce(&mut Rig)) -> Vec<u32> {
    let (mut d, model) = rig();
    let before = model.borrow().watch.steps;
    work(&mut d);
    let m = model.borrow();
    let mut at: Vec<u32> =
        m.watch.regs_at.iter().filter(|&&k| k > before).map(|k| k - before).collect();
    let n = m.watch.steps - before;
    at.extend((1..=n).step_by((n as usize / 24).max(1)));
    at.sort_unstable();
    at.dedup();
    at
}

/// No plane or cursor reached a frame given back, and none is off the
/// frames the driver holds now.
pub fn hold(d: &Rig, model: &Model, w: Wrong) {
    let m = model.borrow();
    confined(&m, &format!("{w:?}"));
    assert_eq!(m.watch.reached, 0, "{w:?}: a plane on a frame given back");
    let held = held(d);
    let on_held = |a: &u64| held.iter().any(|h| (h.0..h.1).contains(a));
    let off: Vec<_> = m.reaching().into_iter().filter(|a| !on_held(a)).collect();
    assert!(
        off.is_empty(),
        "{w:?}: a plane off the frames held: {:x?} of {:x?}",
        &off[..off.len().min(3)],
        held
    );
}
