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

//! Two CORB engines (HDA 1.0a 3.3.18 to 3.3.21). `corb_engine` takes each
//! command as it is posted and answers it; `fetch` reads the ring from
//! CORBRP up to CORBWP while the run bit is set, as a controller does, so a
//! write pointer left behind by a reset shows as commands fetched again.

use nonos_devmodel::FakeBar;

use crate::constants::{CORBCTL, CORBCTL_RUN, CORBRP, CORBWP, RIRBWP, RIRBWP_RST};

/// A controller whose CORB engine is running: it consumes every command the
/// driver posts, shows CORBRP on it and advances RIRBWP to match. RIRBWP
/// belongs to the controller and the driver only reads it, so nothing here
/// races.
pub fn corb_engine(bar: &FakeBar) {
    let wp = bar.wrote16(CORBWP as usize) & 0xff;
    bar.present16(CORBRP as usize, wp);
    bar.present16(RIRBWP as usize, wp);
    // The driver's waits are in milliseconds now; a model that never gives
    // its core back starves the other tests' models past them.
    std::thread::yield_now();
}

/// One pass of a running CORB engine over a 256-entry ring: every command
/// after CORBRP up to CORBWP read from `corb`, CORBRP left on the last, and
/// an answer for each counted on RIRBWP (the responses are those `rings`
/// put in every slot). Nothing while the run bit is clear. The commands
/// read, in order.
pub fn fetch(bar: &FakeBar, corb: &FakeBar) -> Vec<u32> {
    if bar.wrote8(CORBCTL as usize) & CORBCTL_RUN == 0 {
        return Vec::new();
    }
    let wp = bar.wrote16(CORBWP as usize) & 0xff;
    let mut rp = bar.wrote16(CORBRP as usize) & 0xff;
    let rirb = bar.wrote16(RIRBWP as usize);
    // A RIRBWP the driver reset reads as 0 from then on.
    let mut answered = if rirb & RIRBWP_RST != 0 { 0 } else { rirb & 0xff };
    let mut read = Vec::new();
    while rp != wp {
        rp = (rp + 1) & 0xff;
        read.push(corb.wrote32(rp as usize * 4));
        answered = (answered + 1) & 0xff;
    }
    bar.present16(CORBRP as usize, rp);
    bar.present16(RIRBWP as usize, answered);
    read
}
