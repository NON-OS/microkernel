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

//! The monitor on a lit output: its EDID as read (`monitor_read`), kept
//! for the panel the wire describes and the timings a mode set picks.

use super::monitor_read::{read, EDID_MAX};
use super::panel_fits::fits;
use crate::engine::access::{Clock, Mmio};
use crate::engine::gen::{Gen, GEN9};
use crate::engine::pipe::Transcoder;
use crate::engine::route::{self, Route};
use nonos_edid::Edid;
use nonos_gfx_serve::Panel;

/// DVI's and HDMI 1.0's TMDS ceiling, for a sink that names none.
const TMDS_DEFAULT_KHZ: u32 = 165_000;

#[derive(Clone, Copy)]
pub struct Monitor {
    pub blob: [u8; EDID_MAX],
    /// A whole number of blocks, at most EDID_MAX.
    pub len: usize,
}

impl Monitor {
    /// Transcoder `t`'s monitor; read on Gen9 only, whose AUX and GMBUS
    /// registers these are, and empty elsewhere.
    pub fn of(m: &mut dyn Mmio, clock: &dyn Clock, gen: &Gen, t: Transcoder) -> Monitor {
        match route::read(m, t).filter(|_| *gen == GEN9) {
            Some(r) => Monitor::on(m, clock, &r),
            None => Monitor { blob: [0; EDID_MAX], len: 0 },
        }
    }

    /// The monitor at the end of route `r`, lit or dark: a replugged
    /// port's transcoder is off, so its route is the one it was lit on.
    pub fn on(m: &mut dyn Mmio, clock: &dyn Clock, r: &Route) -> Monitor {
        let mut mon = Monitor { blob: [0; EDID_MAX], len: 0 };
        mon.len = read(m, clock, r, &mut mon.blob);
        mon
    }

    pub fn panel(&self) -> Panel {
        Panel::read(&self.blob[..self.len], fits)
    }

    /// The fastest TMDS clock the monitor takes; None when its EDID does
    /// not parse.
    pub fn tmds_khz(&self) -> Option<u32> {
        let edid = Edid::parse(&self.blob[..self.len]).ok()?;
        Some(Some(edid.max_tmds_khz()).filter(|&k| k > 0).unwrap_or(TMDS_DEFAULT_KHZ))
    }
}
