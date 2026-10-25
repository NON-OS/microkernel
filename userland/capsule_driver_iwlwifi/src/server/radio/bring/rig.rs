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

//! What the first bring-up keeps so the firmware can be booted again in the
//! same regions: the device was told their addresses, so they live as long
//! as the capsule does, and a restart reuses them rather than map more.

use crate::firmware::gen3::dev::Dev;
use crate::firmware::gen3::layout::{Board, Memory};
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::select::Transport;
use crate::firmware::gen3::ucode::Ucode;
use crate::firmware::gen3::FwLayout;
use crate::regs::Regs;

use super::super::grant::Grant;
use super::alive::boot_alive;

pub struct Rig {
    pub regs: &'static Regs,
    pub mem: Memory<'static, Grant>,
    pub fw: FwLayout<'static>,
    pub ucode: Ucode<'static>,
    pub board: Board,
    pub transport: Transport,
}

/// Boot the firmware to ALIVE in the rig's regions, on fresh queues (the
/// boot lays the control region out again).
pub fn boot_rig(rig: &'static Rig) -> Result<Dev<'static, Regs, Grant>, Failure> {
    let mut dev = Dev::new(rig.regs, rig.mem.ctrl, rig.mem.rbs);
    boot_alive(&mut dev, &rig.mem, &rig.fw, &rig.ucode, &rig.board)?;
    Ok(dev)
}
