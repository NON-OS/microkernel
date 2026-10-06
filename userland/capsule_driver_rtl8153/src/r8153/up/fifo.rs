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

//! The end of Linux r8153_first_init: VLAN tags, the largest frame, the
//! TX FIFO in auto mode, a second MAC reset, and the FIFO thresholds.

use nonos_usbnet::Bus;

use super::reset::nic_reset;
use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{update_word, write_byte, write_dword, write_word, Dev, PLA};
use crate::r8153::regs::bits::{CPCR_RX_VLAN, TCR0_AUTO_FIFO};
use crate::r8153::regs::pla::{CPCR, MTPS, RMS, RXFIFO_CTRL0, RXFIFO_CTRL1, RXFIFO_CTRL2};
use crate::r8153::regs::pla::{TCR0, TXFIFO_CTRL};

/// rtl8153_change_mtu at the 1500-byte MTU: mtu_to_size adds the VLAN
/// Ethernet header (18) and the FCS (4).
const RX_MAX_SIZE: u16 = 1500 + 18 + 4;
/// MTPS_JUMBO, 12 KiB in 64-byte units.
const MTPS_JUMBO: u8 = (12 * 1024 / 64) as u8;
const RXFIFO_THR1_NORMAL: u32 = 0x0008_0002;
const RXFIFO_THR2_NORMAL: u16 = 0x00a0;
const RXFIFO_THR3_NORMAL: u16 = 0x0110;
const TXFIFO_THR_NORMAL2: u32 = 0x0100_0008;

pub fn fifo<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    // Linux strips VLAN tags into rx_desc when the stack offloads them
    // (NETIF_F_HW_VLAN_CTAG_RX); this stack does not, so they stay in
    // the frame (rtl_rx_vlan_en false).
    at("RX VLAN stripping not off", update_word(dev, PLA, CPCR, CPCR_RX_VLAN, 0))?;
    at("RX frame size refused", write_word(dev, PLA, RMS, RX_MAX_SIZE))?;
    at("TX packet size refused", write_byte(dev, PLA, MTPS, MTPS_JUMBO))?;
    at("TX auto FIFO refused", update_word(dev, PLA, TCR0, 0, TCR0_AUTO_FIFO))?;
    nic_reset(dev)?;
    at("FIFO thresholds refused", thresholds(dev))
}

/// The RX share FIFO credit thresholds, then the TX free credit one.
fn thresholds<B: Bus>(dev: &mut Dev<B>) -> Result<(), i32> {
    write_dword(dev, PLA, RXFIFO_CTRL0, RXFIFO_THR1_NORMAL)?;
    write_word(dev, PLA, RXFIFO_CTRL1, RXFIFO_THR2_NORMAL)?;
    write_word(dev, PLA, RXFIFO_CTRL2, RXFIFO_THR3_NORMAL)?;
    write_dword(dev, PLA, TXFIFO_CTRL, TXFIFO_THR_NORMAL2)
}
