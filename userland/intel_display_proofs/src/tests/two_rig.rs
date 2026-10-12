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

//! Two outputs firmware lit, taken as the capsule takes them: eDP on pipe A
//! at 1920x1080 and DP on pipe B at 1024x768, each in its GGTT window.

use std::{cell::RefCell, rc::Rc};

use super::wire_rig::Rig;
use crate::display::{Head, Intel};
use crate::engine::gen::GEN9;
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::scanout::Scanout;
use crate::engine::takeover::read_output;
use crate::model::firmware::{Axis, T1080};
use crate::model::{Gen9, ModelFrames, Polls, Shared, Ticks};
use nonos_gfx_serve::Panel;
use nonos_gfx_wire::body::state::SCANNING_OUT;

pub const TOTAL: u32 = 1 << 20;
const T768: (Axis, Axis) = ((1024, 24, 136, 1344), (768, 3, 6, 806));

/// GGTT entries per surface, in the 64-entry steps the capsule uses.
pub fn run(w: u32, h: u32) -> u32 {
    ((w * 4).next_multiple_of(64) * h).div_ceil(4096).next_multiple_of(64)
}

fn pages(base: u64, n: u32, px: u32, m: &mut Gen9) -> Vec<u64> {
    m.fill(base, n as u64, px);
    (0..n as u64).map(|i| base + i * 4096).collect()
}

pub fn two_rig() -> (Rig, Rc<RefCell<Gen9>>, [(u32, u32); 2]) {
    let mut m = Gen9::new(TOTAL as usize);
    m.light(Transcoder::Edp, Pipe::A, T1080, 4 << 12);
    m.light(Transcoder::B, Pipe::B, T768, (1 << 28) | (2 << 24) | (3 << 1));
    m.fill(0x9000_0000, 1, 0xFF00_0001);
    crate::engine::ggtt::map(&mut m, Pte::Wide, 0, &[0x9000_0000; 2025], TOTAL).unwrap();
    let a_floor = TOTAL - 2 * run(1920, 1080);
    let windows = [(a_floor, TOTAL), (a_floor - 2 * run(1024, 768), a_floor)];
    let mut heads = [None, None, None, None];
    let specs = [(Pipe::A, 2025, 0x1000_0000, 0xAA), (Pipe::B, 768, 0x3000_0000, 0xCC)];
    let model = Rc::new(RefCell::new(m));
    for (i, (p, n, base, px)) in specs.into_iter().enumerate() {
        let mut m = model.borrow_mut();
        let a = pages(base, n, 0xFF00_0000 | px, &mut m);
        let b = pages(base + 0x0400_0000, n, 0xFF00_0011 + px, &mut m);
        let out = read_output(&mut *m, &GEN9, p).unwrap();
        let scan = Scanout::start(&mut *m, &out, windows[i].1, [&a, &b], &Polls::new(50)).unwrap();
        let (iova, len) = ([a[0], b[0]], n as usize * 4096);
        let model = Shared(model.clone());
        let frames = ModelFrames { model, iova, len, fences: 0, was: None, next: None };
        let (panel, modeset, generation, flags) = (Panel::default(), None, 1, SCANNING_OUT);
        let (window, cursor, bases) = (windows[i], None, iova);
        let h =
            Head { out, scan, frames, panel, modeset, bases, generation, flags, window, cursor };
        heads[i] = Some(h);
    }
    let d = Intel { m: Shared(model.clone()), clock: Ticks::new(), gen: &GEN9, heads };
    (d, model, windows)
}
