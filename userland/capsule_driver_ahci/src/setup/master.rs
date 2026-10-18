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

//! Bus Master at open. A driver that died without its Drop (a panic, a kill,
//! an allocation failure) left its ports' engines armed at frames the kernel
//! freed at the release, which cleared Bus Master. `enable_ahci` has seen the
//! HBA reset end, and that returns every port's PxCMD.ST, CR, FRE and FR to 0
//! (AHCI 1.3.1, 10.4.3); each port the walk reaches is read back stopped too
//! before Bus Master goes on. A port the reset did not stop gets a stop of
//! its own (10.1.2), so one quirky port does not refuse the controller on
//! every retry. A port past the mapped window cannot be read: the reset
//! alone stops it, and the walk never starts it.

use super::pci;
use crate::controller::{armed_port, stop_armed};
use crate::error::{AhciError, AhciResult};
use crate::log::Line;
use crate::regs::Regs;

fn say(port: u8, what: &[u8]) {
    Line::new().text(b"port ").num(port.into()).text(what).send();
}

/// Turn Bus Master on, once every port in `pi` (of the first `count`) reads
/// stopped, a port still armed after the reset stopped first.
/// `EnginesRunning`, Bus Master still off, when one will not stop.
pub(super) fn master_after_reset(
    device_id: u64,
    epoch: u64,
    regs: Regs,
    pi: u32,
    count: u8,
) -> AhciResult<()> {
    if let Some(port) = armed_port(regs, pi, count) {
        say(port, b" still armed after the HBA reset; stopping its engines");
        stop_armed(regs, pi, count);
    }
    if let Some(port) = armed_port(regs, pi, count) {
        say(port, b" still runs after the HBA reset and a stop; controller left alone");
        return Err(AhciError::EnginesRunning);
    }
    pci::command(device_id, epoch, true)
}
