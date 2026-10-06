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

//! The middle of ax88179_reset: bulk IN aggregation, pause levels,
//! checksum offload, the receive filter, and the default medium.

use nonos_usbnet::Bus;

use super::access::{write_mac, Step};
use super::bits::{MEDIUM_DEFAULT, MONITOR_MODE, PAUSE_HIGH, PAUSE_LOW, RX_CTL_ON};
use super::bulkin::{fitted, BULKIN_SIZE};
use super::regs::{MEDIUM_STATUS_MODE, MONITOR_MOD, PAUSE_WATERLVL_HIGH, PAUSE_WATERLVL_LOW};
use super::regs::{RXCOE_CTL, RX_BULKIN_QCTRL, RX_CTL, TXCOE_CTL};

pub(super) fn receive_setup<B: Bus>(bus: &mut B) -> Result<(), Step> {
    // Linux starts from the SuperSpeed gigabit row; the link sets its own
    // once it is up (ax88179_link_reset).
    let bulkin = fitted(BULKIN_SIZE[0]);
    write_mac(bus, RX_BULKIN_QCTRL, &bulkin, "AX_RX_BULKIN_QCTRL write refused")?;
    write_mac(bus, PAUSE_WATERLVL_LOW, &[PAUSE_LOW], "AX_PAUSE_WATERLVL_LOW write refused")?;
    write_mac(bus, PAUSE_WATERLVL_HIGH, &[PAUSE_HIGH], "AX_PAUSE_WATERLVL_HIGH write refused")?;
    // Checksum offload stays off, where Linux turns it on: the stack checks
    // every checksum itself and fills every one it sends.
    write_mac(bus, RXCOE_CTL, &[0], "AX_RXCOE_CTL write refused")?;
    write_mac(bus, TXCOE_CTL, &[0], "AX_TXCOE_CTL write refused")?;
    write_mac(bus, RX_CTL, &RX_CTL_ON.to_le_bytes(), "AX_RX_CTL start refused")?;
    write_mac(bus, MONITOR_MOD, &[MONITOR_MODE], "AX_MONITOR_MOD write refused")?;
    let medium = MEDIUM_DEFAULT.to_le_bytes();
    write_mac(bus, MEDIUM_STATUS_MODE, &medium, "AX_MEDIUM_STATUS_MODE write refused")
}
