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
//! The Voltage Swing Programming Sequence for a combo port (Vol 12 p. 392,
//! 393), before the DDI is enabled and on each change in link training.
//! Group writes are used only for values every lane shares; DW4 carries
//! each lane's loadgen, so it is written lane by lane (p. 393, step 2).

use nonos_intel_display_hw::access::{rmw, Mmio};
use nonos_intel_display_hw::error::Error;

use super::lanes::loadgen;
use super::signal::{pick, Signal};
use crate::regs::clka::Combo;
use crate::regs::phy::{self, group, lane, port};

/// PORT_PCS_DW1 bit 26, cmnkeeper_enable (Vol 2c p. 906).
const CMNKEEPER: u32 = 1 << 26;
/// PORT_CL_DW5 1:0, SUS Clock Config (p. 886).
const SUS_CLOCK: u32 = 3;
/// PORT_TX_DW5 (pp. 943, 944): 31 TX Training Enable, 30 Disable 2tap, 29
/// Disable 3tap, 26 Cursor Program, 25 Coeff Polarity, 20:18 Scaling Mode
/// Sel, 5:3 Rterm Select.
const TRAINING: u32 = 1 << 31;
const DW5_FIELDS: u32 = (1 << 30) | (1 << 29) | (1 << 26) | (1 << 25) | (7 << 18) | (7 << 3);
const DW5_SET: u32 = (1 << 29) | (0b010 << 18) | (0b110 << 3);
/// PORT_TX_DW2 (p. 937): 15 swing_sel upper, 13:11 swing_sel lower, 7:0
/// Rcomp scalar, set to 98h.
const DW2_FIELDS: u32 = (1 << 15) | (7 << 11) | 0xFF;
/// PORT_TX_DW4 (p. 940): 31 Loadgen Select, 17:12 Post Cursor 1, 11:6
/// Post Cursor 2 (set to 0), 5:0 Cursor Coeff.
const LOADGEN: u32 = 1 << 31;
const DW4_FIELDS: u32 = LOADGEN | (0x3F << 12) | (0x3F << 6) | 0x3F;
/// PORT_TX_DW7 30:24, N Scalar (p. 950).
const N_SCALAR: u32 = 0x7F << 24;

/// Programs port `p`'s swing for `sig`. A signal with no row is refused
/// before any write.
pub fn program(m: &mut dyn Mmio, p: Combo, sig: Signal) -> Result<(), Error> {
    let (row, mbps, lanes, dp) = pick(sig)?;
    let gen = loadgen(mbps, lanes).ok_or(Error::Argument)?;
    let shared = |m: &mut dyn Mmio, reg: u32, mask: u32, val: u32| {
        let now = m.read(lane(p, 0, reg));
        m.write(group(p, reg), (now & !mask) | (val & mask));
    };
    shared(m, phy::PCS_DW1, CMNKEEPER, if dp { CMNKEEPER } else { 0 });
    for (n, on) in gen.iter().enumerate() {
        rmw(m, lane(p, n as u32, phy::TX_DW4), LOADGEN, if *on { LOADGEN } else { 0 });
    }
    rmw(m, port(p, phy::CL_DW5), SUS_CLOCK, SUS_CLOCK);
    shared(m, phy::TX_DW5, TRAINING, 0);
    shared(m, phy::TX_DW5, DW5_FIELDS, DW5_SET);
    let sel = ((row.sel >> 3) << 15) | ((row.sel & 7) << 11);
    shared(m, phy::TX_DW2, DW2_FIELDS, sel | 0x98);
    let dw4 = (row.post1 << 12) | row.cursor;
    for (n, on) in gen.iter().enumerate() {
        let keep = if *on { LOADGEN } else { 0 };
        rmw(m, lane(p, n as u32, phy::TX_DW4), DW4_FIELDS, keep | dw4);
    }
    shared(m, phy::TX_DW7, N_SCALAR, row.n_scalar << 24);
    shared(m, phy::TX_DW5, TRAINING, TRAINING);
    Ok(())
}
