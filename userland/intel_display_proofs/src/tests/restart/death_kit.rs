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
//! The death sweeps' rig for a restart (G-78): G-72's DisplayPort rig on a
//! small GGTT, so each restart's rewrite of the table is quick, and the
//! steps of a clean run worth killing the driver at.

use crate::model::firmware::T1080;
use crate::tests::deaths::kit::Model;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::wire_rig::Rig;

/// A small table, so each restart's rewrite of it is quick.
const SMALL: u32 = 1 << 16;

pub fn rig() -> (Rig, Model) {
    let (d, model, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), SMALL);
    (d, model)
}

/// Every register write of a clean run of `work`, and eight steps between.
pub fn steps(work: fn(&mut Rig)) -> Vec<u32> {
    let (mut d, model) = rig();
    let before = model.borrow().watch.steps;
    work(&mut d);
    let m = model.borrow();
    let n = m.watch.steps - before;
    let mut at: Vec<u32> = m.watch.regs_at.iter().filter(|&&k| k > before).map(|k| k - before).collect();
    at.extend((1..=n).step_by((n as usize / 8).max(1)));
    at.sort_unstable();
    at.dedup();
    at
}
