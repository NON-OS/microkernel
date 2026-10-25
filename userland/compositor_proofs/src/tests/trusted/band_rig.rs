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

//! Frames as the tick runs them, for the band proofs: the band rule, then
//! each drained rectangle drawn with the band clipped to it.

use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::band_rule::{due, shows};
use crate::frame_pacer::indicator_band::{band, draw};
use crate::state::indicator::{Indicators, KIND_CAMERA, KIND_MICROPHONE};
use crate::sw_blitter::Surface;

pub(super) const W: u32 = 160;
pub(super) const H: u32 = 40;

#[derive(Clone, Copy, Default)]
pub(super) struct Shows {
    pub(super) ind: Indicators,
    pub(super) captured: bool,
    pub(super) trusted: bool,
}

pub(super) struct Rig {
    pub(super) px: Vec<u32>,
    pub(super) drawn: u32,
    pub(super) acc: DamageAccumulator,
}

fn paint(px: &mut [u32], clip: Rect, s: Shows) {
    let byte_len = px.len() as u64 * 4;
    let dst =
        Surface { base_va: px.as_mut_ptr() as u64, stride: W * 4, width: W, height: H, byte_len };
    draw(dst, clip, &s.ind, s.captured, s.trusted, false);
}

pub(super) fn whole(s: Shows) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    paint(&mut px, Rect { x: 0, y: 0, width: W, height: H }, s);
    px
}

impl Rig {
    pub(super) fn frame(&mut self, s: Shows, rule: bool) {
        let now =
            shows(s.ind.shown(KIND_CAMERA), s.ind.shown(KIND_MICROPHONE), s.captured, s.trusted);
        // Without the rule nothing is watched: a frame as it ran before it.
        if rule && due(core::mem::replace(&mut self.drawn, now), now) {
            self.acc.accumulate(band(W, H));
        }
        while let Some(r) = self.acc.drain() {
            paint(&mut self.px, r, s);
        }
    }
}
