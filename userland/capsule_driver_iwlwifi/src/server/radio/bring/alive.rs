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

//! The firmware booted to ALIVE in the mapped regions, and said so.

use crate::firmware::gen3::bringup::boot;
use crate::firmware::gen3::dev::Dev;
use crate::firmware::gen3::layout::{Board, Memory};
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::prph::read_umac;
use crate::firmware::gen3::regs::{UMAG_SB_CPU_1_STATUS, UMAG_SB_CPU_2_STATUS};
use crate::firmware::gen3::start::stop_device;
use crate::firmware::gen3::ucode::Ucode;
use crate::firmware::gen3::FwLayout;
use crate::regs::Regs;

use super::super::clock::Uptime;
use super::super::grant::Grant;
use super::super::say::Line;
use super::report::say_alive;

pub(super) fn boot_alive(
    dev: &mut Dev<'_, Regs, Grant>,
    mem: &Memory<'_, Grant>,
    fw: &FwLayout<'_>,
    ucode: &Ucode<'_>,
    board: &Board,
) -> Result<(), Failure> {
    let regs = dev.m;
    let mut c = Uptime;
    let alive = match boot(dev, &mut c, mem, fw, ucode, board) {
        Ok(a) => a,
        Err(e) => {
            // What Linux prints when ALIVE does not come: how far the ROM's
            // secure boot got on each CPU.
            let cpu1 = read_umac(regs, &mut c, UMAG_SB_CPU_1_STATUS).unwrap_or(u32::MAX);
            let cpu2 = read_umac(regs, &mut c, UMAG_SB_CPU_2_STATUS).unwrap_or(u32::MAX);
            Line::new()
                .text(b"secure boot status: cpu1 ")
                .hex(cpu1)
                .text(b" cpu2 ")
                .hex(cpu2)
                .send();
            stop_device(regs, &mut c);
            return Err(Failure::Boot(e));
        }
    };
    say_alive(&alive, board.pnvm.is_some(), dev.pnvm);
    Ok(())
}
