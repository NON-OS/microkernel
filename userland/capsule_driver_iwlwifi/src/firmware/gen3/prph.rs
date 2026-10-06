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

//! Peripheral register access on AX210-family parts. A PRPH register is
//! reached through the HBUS window: write the address (with the byte-enable
//! bits) to WADDR or RADDR, then move the value through WDAT or RDAT. AX210
//! and newer decode 24 address bits (`iwl_trans_pcie_prph_msk`); the legacy
//! 20-bit mask the older load path uses would cut every UMAC register at
//! 0xD0_0000 and up. UMAC registers sit `UMAC_PRPH_OFFSET` above their base.
//! Access needs the MAC awake: `grab` raises MAC_ACCESS_REQ and waits for the
//! clock (Linux `__iwl_trans_pcie_grab_nic_access`, 15 ms), `release` drops it.

use super::region::Clock;
use super::regs::{
    CSR_GP_CNTRL, GP_GOING_TO_SLEEP, GP_MAC_ACCESS_REQ, GP_MAC_CLOCK_READY, HBUS_TARG_PRPH_RADDR,
    HBUS_TARG_PRPH_RDAT, HBUS_TARG_PRPH_WADDR, HBUS_TARG_PRPH_WDAT,
};
use crate::regs::Mmio;

/// `umac_prph_offset` of the AX210 transport configs.
pub const UMAC_PRPH_OFFSET: u32 = 0x30_0000;
const PRPH_MASK: u32 = 0x00FF_FFFF;
const BYTE_ENABLE: u32 = 3 << 24;
const GRAB_MS: u32 = 15;

/// Wake the MAC for register access. False if the clock never came up.
pub fn grab<M: Mmio, C: Clock>(m: &M, c: &mut C) -> bool {
    m.write32(CSR_GP_CNTRL, m.read32(CSR_GP_CNTRL) | GP_MAC_ACCESS_REQ);
    c.delay_us(2);
    c.poll_for(GRAB_MS, &mut || {
        m.read32(CSR_GP_CNTRL) & (GP_MAC_CLOCK_READY | GP_GOING_TO_SLEEP) == GP_MAC_CLOCK_READY
    })
}

/// Let the MAC sleep again.
pub fn release<M: Mmio>(m: &M) {
    m.write32(CSR_GP_CNTRL, m.read32(CSR_GP_CNTRL) & !GP_MAC_ACCESS_REQ);
}

pub fn write_no_grab<M: Mmio>(m: &M, addr: u32, val: u32) {
    m.write32(HBUS_TARG_PRPH_WADDR, (addr & PRPH_MASK) | BYTE_ENABLE);
    m.write32(HBUS_TARG_PRPH_WDAT, val);
}

pub fn read_no_grab<M: Mmio>(m: &M, addr: u32) -> u32 {
    m.write32(HBUS_TARG_PRPH_RADDR, (addr & PRPH_MASK) | BYTE_ENABLE);
    m.read32(HBUS_TARG_PRPH_RDAT)
}

/// Write a UMAC register, holding the MAC awake for it.
pub fn write_umac<M: Mmio, C: Clock>(m: &M, c: &mut C, addr: u32, val: u32) -> bool {
    if !grab(m, c) {
        return false;
    }
    write_no_grab(m, addr.wrapping_add(UMAC_PRPH_OFFSET), val);
    release(m);
    true
}

/// Read a UMAC register, or `None` if the MAC did not wake.
pub fn read_umac<M: Mmio, C: Clock>(m: &M, c: &mut C, addr: u32) -> Option<u32> {
    if !grab(m, c) {
        return None;
    }
    let v = read_no_grab(m, addr.wrapping_add(UMAC_PRPH_OFFSET));
    release(m);
    Some(v)
}
