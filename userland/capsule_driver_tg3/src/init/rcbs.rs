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

//! tg3_rings_reset for one vector on a SUPPORT_MSIX part: extra rings off,
//! every vector masked with its mailboxes zeroed, the status block cleared
//! and given, then the send and return control blocks in the part's memory.

use crate::chip::sram;
use crate::constants::hostcc::{HOSTCC_STATUS_BLK_HOST_ADDR, IRQ_MAX_VECS};
use crate::constants::layout::STATUS_AT;
use crate::constants::mailbox::*;
use crate::constants::ring::*;
use crate::constants::sram::{NIC_SRAM_RCV_RET_RCB, NIC_SRAM_SEND_RCB};
use crate::regs::Regs;
use crate::setup::Driver;

/// The MAXLEN_FLAGS word of every ring past the first the 57765 class has:
/// one send ring and three return rings, each to be disabled.
pub fn disabled() -> impl Iterator<Item = u32> {
    let send = (1..SEND_RCBS).map(|n| NIC_SRAM_SEND_RCB + n * TG3_BDINFO_SIZE);
    let ret = (1..RET_RCBS).map(|n| NIC_SRAM_RCV_RET_RCB + n * TG3_BDINFO_SIZE);
    send.chain(ret).map(|rcb| rcb + TG3_BDINFO_MAXLEN_FLAGS)
}

/// tg3_set_bdinfo on a 5705_PLUS part: the NIC address is not written.
fn bdinfo(regs: &Regs, rcb: u32, mapping: u64, maxlen_flags: u32) {
    let host = rcb + TG3_BDINFO_HOST_ADDR;
    sram::write(regs, host + TG3_64BIT_REG_HIGH as u32, (mapping >> 32) as u32);
    sram::write(regs, host + TG3_64BIT_REG_LOW as u32, mapping as u32);
    sram::write(regs, rcb + TG3_BDINFO_MAXLEN_FLAGS, maxlen_flags);
}

/*
 * SAFETY: `d.regs` carries the broker MmioMap base for BAR0; every mailbox
 * and HOSTCC_STATUS_BLK_HOST_ADDR is a 32-bit register inside it.
 */
pub fn reset(d: &mut Driver) {
    let regs = d.regs;
    for at in disabled() {
        sram::write(&regs, at, BDINFO_FLAGS_DISABLED);
    }
    unsafe {
        regs.w32f(int_mbox(0), 1);
        for n in 1..IRQ_MAX_VECS {
            regs.w32(consumer_mbox(n), 0);
            regs.w32f(int_mbox(n), 1);
        }
        regs.w32(SNDHOST_PROD_MBOX_0, 0);
    }
    d.tx.init();
    d.status.clear();
    let status = d.rings_device_addr + STATUS_AT as u64;
    unsafe {
        regs.w32(HOSTCC_STATUS_BLK_HOST_ADDR + TG3_64BIT_REG_HIGH, (status >> 32) as u32);
        regs.w32(HOSTCC_STATUS_BLK_HOST_ADDR + TG3_64BIT_REG_LOW, status as u32);
    }
    let tx = d.tx.ring_device_addr;
    bdinfo(&regs, NIC_SRAM_SEND_RCB, tx, (TX_RING_SIZE as u32) << BDINFO_FLAGS_MAXLEN_SHIFT);
    let ret = d.rx.ret_ring_device_addr;
    let ret_len = (RX_RET_RING_SIZE as u32) << BDINFO_FLAGS_MAXLEN_SHIFT;
    bdinfo(&regs, NIC_SRAM_RCV_RET_RCB, ret, ret_len);
}
