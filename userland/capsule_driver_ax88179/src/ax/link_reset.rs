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

//! ax88179_link_reset: once the PHY has a link, the receiver restarted
//! until the USB TX FIFO reads empty, then the bulk IN row and the medium
//! mode set for the speed and duplex the PHY resolved.

use nonos_libc::Deadline;
use nonos_usbnet::Bus;

use super::access::{read, read_mac, read_phy, write_mac, Step};
use super::bits::{RX_CTL_ON, RX_CTL_STOP};
use super::medium::medium;
use super::phy_regs::PHYSR;
use super::regs::{MEDIUM_STATUS_MODE, PHYSICAL_LINK_STATUS, RX_BULKIN_QCTRL, RX_CTL};
use super::regs::{TX_FIFO_BUSY, TX_FIFO_REQUEST, TX_FIFO_VALUE};

/// Linux gives the FIFO HZ / 10.
const FIFO_WAIT_MS: u64 = 100;

/// The PHYSR the medium was set for, or `None` when Linux would leave the
/// carrier off: the FIFO did not drain in time, or the link went again.
pub(super) fn link_reset<B: Bus>(bus: &mut B) -> Result<Option<u16>, Step> {
    let deadline = Deadline::after_ms(FIFO_WAIT_MS);
    loop {
        let stop = RX_CTL_STOP.to_le_bytes();
        write_mac(bus, RX_CTL, &stop, "AX_RX_CTL stop refused")?;
        write_mac(bus, RX_CTL, &RX_CTL_ON.to_le_bytes(), "AX_RX_CTL start refused")?;
        let mut fifo = [0u8; 4];
        read(bus, (TX_FIFO_REQUEST, TX_FIFO_VALUE, 0), &mut fifo, "TX FIFO state unread")?;
        if u32::from_le_bytes(fifo) & TX_FIFO_BUSY == 0 {
            break;
        }
        if deadline.expired() {
            return Ok(None);
        }
    }
    let mut sts = [0u8; 1];
    read_mac(bus, PHYSICAL_LINK_STATUS, &mut sts, "PHYSICAL_LINK_STATUS unread")?;
    let physr = read_phy(bus, PHYSR, "PHY PHYSR unread")?;
    let Some(m) = medium(physr, sts[0]) else { return Ok(None) };
    write_mac(bus, RX_BULKIN_QCTRL, &m.bulkin, "AX_RX_BULKIN_QCTRL write refused")?;
    let mode = m.mode.to_le_bytes();
    write_mac(bus, MEDIUM_STATUS_MODE, &mode, "AX_MEDIUM_STATUS_MODE write refused")?;
    Ok(Some(physr))
}
