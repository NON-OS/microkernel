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

//! Powering the root ports: unless the root hub says its ports are always
//! powered, global power and each port's power are switched on (one of the
//! two applies, by the hub's switching mode), then power-on to power-good
//! is waited for. Returns the number of ports.

use crate::env::{hold, Clock, Regs};
use crate::facts::bounds::POWER_MIN_MS;
use crate::facts::port::*;

pub fn power<R: Regs, C: Clock>(r: &mut R, c: &C) -> u8 {
    let a = r.r32(HC_RH_DESCRIPTOR_A);
    let ports = ((a & RHA_NDP) as u8).min(PORTS_MAX);
    if a & RHA_NPS == 0 {
        r.w32(HC_RH_STATUS, RHS_LPSC);
        for p in 1..=ports {
            r.w32(super::port::offset(p), PORT_SET_POWER);
        }
        let good = 2 * (a >> RHA_POTPGT_SHIFT) as u64;
        hold(c, good.max(POWER_MIN_MS));
    }
    ports
}
