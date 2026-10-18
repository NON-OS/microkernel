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

//! The link out of a cold reset, as Linux's intel8x0 takes it: the link on,
//! two channels, interrupts off, cold reset asserted and released, then the
//! primary codec's ready bit. A part already out of reset (firmware played
//! through it) is reset again, so the codec starts from its defaults.

use crate::constants::bus::{
    GC_ACLINK_OFF, GC_COLD, GC_GIE, GC_PCM_246, GLOB_CNT, GLOB_STA, GS_PCR,
};
use crate::regs::Port;
use crate::wait::{hold, until};

/// AC'97 2.3 gives the cold reset pulse a microsecond; a millisecond covers
/// it and the codec's clock coming back.
const COLD_RESET_MS: u64 = 1;
/// Linux waits a second for any codec to say it is ready.
const CODEC_READY_MS: u64 = 1000;

pub fn bring_up(bus: &Port) -> Result<(), &'static str> {
    let cnt = bus.read32(GLOB_CNT) & !(GC_ACLINK_OFF | GC_PCM_246 | GC_GIE);
    bus.write32(GLOB_CNT, cnt & !GC_COLD);
    hold(COLD_RESET_MS);
    bus.write32(GLOB_CNT, cnt | GC_COLD);
    hold(COLD_RESET_MS);
    if !until(CODEC_READY_MS, || bus.read32(GLOB_STA) & GS_PCR != 0) {
        return Err("no codec ready on the AC-link");
    }
    Ok(())
}
