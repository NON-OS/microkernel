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

//! A monitor pulled, and one back. A pulled output is darkened as a mode
//! set darkens it (plane, transcoder, port, its DPLL), so nothing drives a
//! cable with no monitor, and answers dark until it is back; a darkening
//! that times out part way leaves it Out all the same, since the monitor
//! is gone and the next lighting starts from dark. One back has its EDID
//! read again on the route it was lit on (another monitor may be in the
//! port): the panel, modes and TMDS ceiling the wire describes come from
//! it, DDC/CI starts afresh for it, and it is Back until a mode set lights
//! it. One whose EDID does not answer yet stays Out for the next poll.

use super::frames::Frames;
use super::intel::Intel;
use super::monitor::Monitor;
use super::plug::Plug;
use crate::ddc::pace::Pacer;
use crate::engine::access::{Clock, Mmio};
use crate::engine::modeset_off::darken;
use crate::engine::route::Route;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub(super) fn pulled(&mut self, o: usize) {
        let Intel { m, clock, heads, .. } = self;
        let Some(h) = heads[o].as_mut() else { return };
        if let Some(ms) = h.modeset {
            let _ = darken(m, clock, (ms.lit.t, ms.lit.p, ms.lit.ddi, ms.lit.dpll));
        }
        (h.plug, h.flags, h.generation) = (Plug::Out, 0, h.generation.wrapping_add(1));
    }

    pub(super) fn replugged(&mut self, o: usize) {
        let Intel { m, clock, heads, ddc, .. } = self;
        let Some(h) = heads[o].as_mut() else { return };
        let Some(ms) = h.modeset.as_mut() else { return };
        let mon = Monitor::on(m, clock, &Route { ddi: ms.lit.ddi, link: ms.lit.link });
        let Some(tmds_khz) = mon.tmds_khz() else { return };
        (ms.monitor, ms.tmds_khz, h.panel) = (mon, tmds_khz, mon.panel());
        ddc[o] = Pacer::default();
        (h.plug, h.generation) = (Plug::Back, h.generation.wrapping_add(1));
    }
}
