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

//! The EDID of the monitor on a lit output, read the way its port speaks:
//! I2C over the DisplayPort AUX channel (VESA DisplayPort 1.4, section
//! 2.7.7) for DisplayPort and eDP, DDC through GMBUS for HDMI and DVI.
//! The base block, then the one extension a single 256-byte segment holds;
//! a monitor that does not answer gives an empty blob, never an error.

use super::panel_fits::fits;
use crate::dp::i2c::read_edid as aux_edid;
use crate::engine::access::{Clock, Mmio};
use crate::engine::aux_regs::Ddi;
use crate::engine::gen::{Gen, GEN9};
use crate::engine::gmbus::read_edid as ddc_edid;
use crate::engine::pipe::Transcoder;
use crate::engine::port::Port;
use crate::engine::route::{self, Link, Route};
use nonos_gfx_serve::Panel;

const HEADER: [u8; 8] = [0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0];

/// The bytes read into `out`: 0, 128 or 256.
pub fn read(m: &mut dyn Mmio, clock: &dyn Clock, r: &Route, out: &mut [u8; 256]) -> usize {
    let (base, ext) = out.split_at_mut(128);
    if block(m, clock, r, 0, base).is_none() || base[..8] != HEADER {
        return 0;
    }
    if base[126] == 0 || block(m, clock, r, 128, ext).is_none() {
        return 128;
    }
    256
}

fn block(m: &mut dyn Mmio, clock: &dyn Clock, r: &Route, at: u8, out: &mut [u8]) -> Option<()> {
    match r.link {
        Link::Dp { .. } => {
            let ddi = [Ddi::A, Ddi::B, Ddi::C, Ddi::D].get(r.ddi as usize).copied()?;
            let mut p = Port { mmio: m, ddi, dpll: 0, enhanced: false, clock };
            aux_edid(&mut p, at, out).ok()
        }
        Link::Hdmi | Link::Dvi => ddc_edid(m, clock, r.ddi, at, out).ok(),
    }
}

/// The panel on route `r`, empty when its monitor says nothing.
pub fn panel(m: &mut dyn Mmio, clock: &dyn Clock, r: &Route) -> Panel {
    let mut blob = [0u8; 256];
    let n = read(m, clock, r, &mut blob);
    Panel::read(&blob[..n], fits)
}

/// The panel on transcoder `t`'s monitor; read on Gen9 only, whose AUX
/// and GMBUS registers these are, and empty elsewhere.
pub fn panel_of(m: &mut dyn Mmio, clock: &dyn Clock, gen: &Gen, t: Transcoder) -> Panel {
    match route::read(m, t).filter(|_| *gen == GEN9) {
        Some(r) => panel(m, clock, &r),
        None => Panel::default(),
    }
}
