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
//! The timing written into a transcoder that is off.

use nonos_edid::Detailed;
use nonos_intel_display_hw::access::Mmio;
use nonos_intel_display_hw::error::Error;
use nonos_intel_display_hw::pipe::{Pipe, Transcoder};

use super::check::Sink;
use super::words::{words, Words};
use crate::regs::trans::{self, of};

/// Checks `t`, then writes it into transcoder `tr` and pipe `p`'s source
/// size. Vol 2c says the timing should not change while the transcoder is
/// enabled (TRANS_HTOTAL, p. 1365), so a TRANS_CONF reading enabled or
/// still running is refused, after that one read and before any write.
pub fn program(
    m: &mut dyn Mmio,
    tr: Transcoder,
    p: Pipe,
    t: &Detailed,
    sink: Sink,
) -> Result<Words, Error> {
    let w = words(t, sink)?;
    let regs = [
        (of(trans::HTOTAL, tr)?, w.htotal),
        (of(trans::HBLANK, tr)?, w.hblank),
        (of(trans::HSYNC, tr)?, w.hsync),
        (of(trans::VTOTAL, tr)?, w.vtotal),
        (of(trans::VBLANK, tr)?, w.vblank),
        (of(trans::VSYNC, tr)?, w.vsync),
        (trans::srcsz(p), w.srcsz),
    ];
    let conf = m.read(of(trans::CONF, tr)?);
    if conf & (trans::CONF_ENABLE | trans::CONF_STATE) != 0 {
        return Err(Error::Argument);
    }
    for (reg, value) in regs {
        m.write(reg, value);
    }
    Ok(w)
}
