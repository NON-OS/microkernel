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

//! Linux rtl8153_enable and rtl_enable: the gap for the link's speed, on
//! RTL_VER_09 the flow-control patch task restarted, the packet filter
//! reset, TX and RX on, on the RTL8153B the RX DMA owner updated, and the
//! RX gate opened; then the RX mode.

use nonos_libc::mk_idle_ms;
use nonos_usbnet::Bus;

use super::ifg::ifg;
use super::rx_mode::rx_mode;
use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{read_word, update_byte, write_byte, write_word, Dev, PLA, USB};
use crate::r8153::regs::bits::{CR_RE, CR_TE, FMC_FCR_MCU_EN};
use crate::r8153::regs::pla::{CR, FMC};
use crate::r8153::regs::usb::{FC_PATCH_TASK, FW_TASK, OWN_CLEAR, OWN_UPDATE, UPT_RXDMA_OWN};
use crate::r8153::up::rxdy_gate;
use crate::r8153::Version;

pub fn enable<B: Bus>(dev: &mut Dev<B>, v: Version, speed: u16) -> Result<(), Fail> {
    at("inter-frame gap refused", ifg(dev, speed))?;
    if v == Version::V09 {
        at("flow-control task not restarted", toggle(dev, USB, FW_TASK, FC_PATCH_TASK, 2))?;
    }
    // r8152b_reset_packet_filter.
    at("packet filter not reset", toggle(dev, PLA, FMC, FMC_FCR_MCU_EN, 0))?;
    at("TX and RX not enabled", update_byte(dev, PLA, CR, 0, CR_RE | CR_TE))?;
    if v.is_8153b() {
        // r8153b_rx_agg_chg_indicate.
        let own = write_byte(dev, USB, UPT_RXDMA_OWN, OWN_UPDATE | OWN_CLEAR);
        at("RX DMA owner not updated", own)?;
    }
    at("RX gate not opened", rxdy_gate(dev, false))?;
    at("RX mode refused", rx_mode(dev))
}

/// `bit` cleared then set again from one read, `pause_ms` apart (Linux
/// sleeps 1 to 2 ms between the two FC_PATCH_TASK writes, none for FMC).
fn toggle<B: Bus>(
    dev: &mut Dev<B>,
    ty: u16,
    addr: u16,
    bit: u16,
    pause_ms: u64,
) -> Result<(), i32> {
    let v = read_word(dev, ty, addr)?;
    write_word(dev, ty, addr, v & !bit)?;
    if pause_ms > 0 {
        let _ = mk_idle_ms(pause_ms);
    }
    write_word(dev, ty, addr, v | bit)
}
