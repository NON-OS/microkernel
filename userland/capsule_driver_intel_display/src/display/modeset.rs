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

//! What a mode set on a taken-over output needs, found at bring-up: Gen9
//! (whose mode setting is written), a pipe transcoder on DisplayPort or
//! HDMI (eDP keeps its panel's mode on the boot DPLL; DVI is not driven),
//! the DPLL its DDI runs on (never the boot one), the monitor's EDID with
//! the timing shown among its own, pcode's latencies, its GGTT window.
//! Without any, the output keeps firmware's mode and no MODE_SET.

use super::monitor::Monitor;
use super::timing_pick::matching;
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::gen::{Gen, GEN9};
use crate::engine::pcode::latencies;
use crate::engine::pipe::Transcoder;
use crate::engine::relight::Lit;
use crate::engine::route::{self, Link};
use crate::engine::takeover::Output;
use crate::engine::timing;
use nonos_edid::Detailed;

/// pcode answers its mailbox in a few milliseconds (unverified).
const PCODE_US: u64 = 50_000;

#[derive(Clone, Copy)]
pub struct Modeset {
    pub lit: Lit,
    pub monitor: Monitor,
    pub tmds_khz: u32,
    /// The timing shown now, clock included.
    pub now: Detailed,
    pub latency_us: [u32; 8],
    /// The GGTT entries this output's surfaces may use, first to last + 1.
    pub window: (u32, u32),
}

impl Modeset {
    pub fn find(
        m: &mut dyn Mmio,
        clock: &dyn Clock,
        gen: &Gen,
        out: &Output,
        monitor: &Monitor,
        window: (u32, u32),
    ) -> Option<Modeset> {
        let gen9 = *gen == GEN9 && out.transcoder != Transcoder::Edp && monitor.len > 0;
        let r = route::read(m, out.transcoder).filter(|_| gen9)?;
        if !matches!(r.link, Link::Dp { .. } | Link::Hdmi) {
            return None;
        }
        let dpll = route::dpll_of(m, r.ddi).filter(|&d| d != 0)?;
        let now = matching(&monitor.blob[..monitor.len], &timing::read(m, out.transcoder)?)?;
        let latency_us = latencies(m, &Until::new(clock, PCODE_US)).ok()?;
        let tmds_khz = monitor.tmds_khz()?;
        let lit = Lit { t: out.transcoder, p: out.pipe, ddi: r.ddi, dpll, link: r.link };
        Some(Modeset { lit, monitor: *monitor, tmds_khz, now, latency_us, window })
    }
}
