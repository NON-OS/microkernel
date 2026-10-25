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

//! One try at an EDID block, the way the port speaks: I2C over the AUX
//! channel for DisplayPort and eDP, DDC through GMBUS for HDMI and DVI,
//! each through E-DDC's segment pointer and within one transaction's
//! deadline or the whole read's, whichever ends first.

use crate::dp::i2c_edid::read_edid as aux_edid;
use crate::engine::access::{Clock, Mmio};
use crate::engine::aux_regs::Ddi;
use crate::engine::gmbus::{read_block, BLOCK};
use crate::engine::gmbus_xfer::deadline::{Deadline, XFER_US};
use crate::engine::port::Port;
use crate::engine::route::{Link, Route};

/// One try at block `n`, from segment n / 2 on either port.
pub fn once(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    r: &Route,
    n: u8,
    out: &mut [u8; BLOCK],
    whole: &Deadline,
) -> Option<()> {
    match r.link {
        Link::Dp { .. } => {
            let ddi = [Ddi::A, Ddi::B, Ddi::C, Ddi::D].get(r.ddi as usize).copied()?;
            let mut p = Port { mmio: m, ddi, dpll: 0, enhanced: false, clock };
            aux_edid(&mut p, n / 2, n % 2 * 128, out, &whole.within(XFER_US)).ok()
        }
        Link::Hdmi | Link::Dvi => read_block(m, clock, r.ddi, n, out, &whole.within(XFER_US)).ok(),
    }
}
