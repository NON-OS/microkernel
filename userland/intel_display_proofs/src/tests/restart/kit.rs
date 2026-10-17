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
//! A restart's takeover on the model as the capsule's `take` runs it
//! (G-78): the first lit output started with the plan's `ours` on fresh
//! frames, then every lit output settled (`settle_heads`). The watch is
//! armed afresh with the new frames; what the dead driver gave back stays
//! given back, and nothing may reach it.

use std::cell::RefCell;
use std::rc::Rc;

use crate::display::{settle_heads, Intel};
use crate::engine::gen::GEN9;
use crate::engine::ggtt_pte::Pte;
use crate::engine::scanout::Scanout;
use crate::engine::takeover_dark::read_outputs_kept;
use crate::model::{Gen9, ModelFrames, Polls, Shared, Ticks};
use crate::tests::wire_rig::{taken, Rig};

pub const TOTAL: u32 = 1 << 20;
/// The restarted driver's two surfaces, filled 0xAA and 0xBB.
pub const NEW: [u64; 2] = [0x5000_0000, 0x5800_0000];

pub fn pages(base: u64, n: u32) -> Vec<u64> {
    (0..n as u64).map(|i| base + i * 4096).collect()
}

/// The restart on what `m` holds, the plan's `ours` half its table; None
/// when nothing is lit to take.
pub fn restart(mut m: Gen9) -> Option<(Rig, Rc<RefCell<Gen9>>)> {
    (m.watch.die_at, m.watch.dead_at, m.watch.reached) = (None, None, 0);
    m.confine = Default::default();
    let total = m.ggtt.len() as u32;
    let lit = read_outputs_kept(&mut m, &GEN9, total);
    let out = lit.iter().flatten().next().copied()?;
    let n = ((out.width * 4).next_multiple_of(64) * out.height).div_ceil(4096);
    m.fill(NEW[0], n as u64, 0xFF00_00AA);
    m.fill(NEW[1], n as u64, 0xFF00_00BB);
    crate::tests::confine::arm(&mut m, &NEW, n as usize);
    let iovas = [pages(NEW[0], n), pages(NEW[1], n)];
    let ours = total / 2;
    let scan = Scanout::start(&mut m, &out, total, [&iovas[0], &iovas[1]], ours, &Polls::new(50));
    let model = Rc::new(RefCell::new(m));
    let shared = Shared(model.clone());
    let (len, next) = (n as usize * 4096, None);
    let frames = ModelFrames { model: shared.clone(), iova: NEW, len, fences: 0, was: None, next };
    let head = taken(out, scan.expect("taken"), frames, (Default::default(), None), (0, total));
    let mut d = Intel::new(shared, Ticks::new(), &GEN9, [Some(head), None, None, None]);
    settle_heads(&mut d.m, &lit, &d.heads, (Pte::Wide, ours, total), &Polls::new(50));
    Some((d, model))
}
