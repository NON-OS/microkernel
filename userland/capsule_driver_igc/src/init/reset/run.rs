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

//! The reset in igc_reset_hw_base's order. Firmware (PXE, UEFI UNDI) or a
//! previous OS can leave the queues running, and a reset landing on a
//! bus-master cycle still in flight is what the master stop exists to
//! prevent, so nothing is reset until that stop is seen.

use crate::regs::Regs;

use super::{global, master, quiesce};

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    master::stop(regs)?;
    quiesce::run(regs);
    global::run(regs);
    Ok(())
}
