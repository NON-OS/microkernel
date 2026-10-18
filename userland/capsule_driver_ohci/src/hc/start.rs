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

//! Bringing the controller up: taken from firmware, every interrupt off
//! (the driver polls), the frame interval kept, the schedule built, a host
//! reset seen to clear, then within the 2 ms the suspended controller
//! allows (spec, as recalled): the HCCA and list heads given, the frame
//! interval, periodic start and low-speed threshold set, every status
//! cleared, and the controller made operational with all lists on.

use super::own::take;
use crate::env::{until, Clock, Mem, Regs};
use crate::facts::bounds::HCR_MS;
use crate::facts::regs::*;
use crate::sched::layout::{bulk_ed, CTRL_ED, HCCA};
use crate::sched::lists::build;

pub const HCR_STUCK: &str = "HCR did not clear";
pub const NOT_OPERATIONAL: &str = "controller did not become operational";

pub fn start<R: Regs, C: Clock, M: Mem>(
    r: &mut R,
    c: &C,
    m: &M,
    mapped: u64,
) -> Result<(), &'static str> {
    take(r, c, mapped)?;
    r.w32(HC_INTERRUPT_DISABLE, INT_ALL);
    let fi = match r.r32(HC_FM_INTERVAL) & FI_MASK {
        0 => FI_DEFAULT,
        fi => fi,
    };
    build(m);
    r.w32(HC_COMMAND_STATUS, CS_HCR);
    if !until(c, HCR_MS, || r.r32(HC_COMMAND_STATUS) & CS_HCR == 0) {
        return Err(HCR_STUCK);
    }
    r.w32(HC_HCCA, m.bus(HCCA));
    r.w32(HC_CONTROL_HEAD_ED, m.bus(CTRL_ED));
    r.w32(HC_CONTROL_CURRENT_ED, 0);
    r.w32(HC_BULK_HEAD_ED, m.bus(bulk_ed(0)));
    r.w32(HC_BULK_CURRENT_ED, 0);
    // The largest data packet: the frame less the bit-stuffing and SOF
    // overhead (6/7 of FI - 210, spec, as recalled).
    let fsmps = (6 * (fi - 210)) / 7;
    let fit = (r.r32(HC_FM_INTERVAL) & FIT) ^ FIT;
    r.w32(HC_FM_INTERVAL, fit | fsmps << FSMPS_SHIFT | fi);
    r.w32(HC_PERIODIC_START, fi * 9 / 10);
    r.w32(HC_LS_THRESHOLD, LS_THRESHOLD);
    r.w32(HC_INTERRUPT_STATUS, INT_ALL);
    r.w32(HC_CONTROL, HCFS_OPERATIONAL | CTL_PLE | CTL_CLE | CTL_BLE | CTL_CBSR_4);
    if r.r32(HC_CONTROL) & CTL_HCFS != HCFS_OPERATIONAL {
        return Err(NOT_OPERATIONAL);
    }
    Ok(())
}
