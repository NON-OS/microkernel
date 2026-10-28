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

//! Running the bring-up plan (plan.rs) on a configured device, then
//! autonegotiation restarted from the PHY's own control word. Every wait
//! is a fixed pause on the clock, never a poll that could go on: the whole
//! bring-up sends 21 requests and waits 700 ms, inside the bind's budget.

use nonos_usbnet::Bus;

use super::plan::plan;
use super::regs::{AUTONEG_RESTART, PHY_CONTROL};
use super::step::Step;
use super::vendor::{mac_out, phy_in, phy_out};

/// A bus that can wait: the kernel's clock on driver.xhci0, a count of
/// what was waited in the proofs.
pub trait Pause {
    fn pause(&mut self, ms: u64);
}

pub type Stop = (&'static str, i32);

pub fn bring_up<B: Bus + Pause>(bus: &mut B, station: [u8; 6]) -> Result<(), Stop> {
    for (what, step) in plan(station) {
        run(bus, step).map_err(|e| (what, e))?;
    }
    let control = phy_in(bus, PHY_CONTROL).map_err(|e| ("PHY control unread", e))?;
    let restart = control | AUTONEG_RESTART;
    phy_out(bus, PHY_CONTROL, restart).map_err(|e| ("autonegotiation refused", e))
}

fn run<B: Bus + Pause>(bus: &mut B, step: Step) -> Result<(), i32> {
    match step {
        Step::Mac { reg, data, len } => mac_out(bus, reg, &data[..len as usize]),
        Step::Phy { reg, value } => phy_out(bus, reg, value),
        Step::Pause(ms) => {
            bus.pause(ms);
            Ok(())
        }
    }
}
