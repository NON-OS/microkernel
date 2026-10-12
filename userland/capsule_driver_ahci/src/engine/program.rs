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

use super::cmd_header::CmdHeader;
use super::port::Port;
use crate::constants::regs::{
    CMD_FRE, CMD_POD, CMD_SUD, PORT_CLB, PORT_CLBU, PORT_CMD, PORT_FB, PORT_FBU, PORT_IE,
    PORT_IE_DEFAULT, PORT_IS, PORT_SCTL, PORT_SERR, SCTL_IPM_DISABLE,
};
use crate::regs::Regs;

/// Point the port's command list and FIS area at its regions, slot 0's
/// header at its command table, clear its latched errors, and turn FIS
/// receive on.
pub(super) fn program(regs: Regs, port: &Port) {
    let (base, clb_phys, fb_phys) = (port.base, port.clb.device_addr(), port._fb.device_addr());
    let ctba_phys = port.ctba.device_addr();
    let header = CmdHeader {
        flags: 0,
        pm: 0,
        prdtl: 0,
        prdbc: 0,
        ctba_low: ctba_phys as u32,
        ctba_high: (ctba_phys >> 32) as u32,
        rsv: [0; 4],
    };
    // SAFETY: the command list is mapped for the port's life with room for
    // slot 0's header, and the engine is stopped, so the HBA reads none of
    // it; the port's register block lies inside the mapped ABAR.
    unsafe {
        core::ptr::write_volatile(port.clb.user_va() as *mut CmdHeader, header);
        regs.w32(base + PORT_CLB, clb_phys as u32);
        regs.w32(base + PORT_CLBU, (clb_phys >> 32) as u32);
        regs.w32(base + PORT_FB, fb_phys as u32);
        regs.w32(base + PORT_FBU, (fb_phys >> 32) as u32);
        regs.w32(base + PORT_IS, regs.r32(base + PORT_IS));
        regs.w32(base + PORT_IE, PORT_IE_DEFAULT);
        regs.w32(base + PORT_SERR, regs.r32(base + PORT_SERR));
        regs.w32(base + PORT_SCTL, regs.r32(base + PORT_SCTL) | SCTL_IPM_DISABLE);
        // Enable FIS receive (FRE) as part of bring-up. FRE must be set before
        // ST, which start() sets only after the link is up; setting them
        // together is out of spec and some controllers reject it.
        regs.w32(base + PORT_CMD, regs.r32(base + PORT_CMD) | CMD_FRE | CMD_POD | CMD_SUD);
    }
}
