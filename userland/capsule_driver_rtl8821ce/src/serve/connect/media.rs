/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! Linking the MAC to the network's BSS before the handshake.

use crate::constants::regs::{NETTYPE_MASK, NETTYPE_SHIFT, NET_TYPE_LINKED, REG_BSSID, REG_CR};
use crate::regs::{Mmio, Regs};

/*
 * Put the MAC into infrastructure "linked" mode for one BSS so the receiver
 * accepts the access point's unicast data frames (the EAPOL handshake) addressed
 * to us. Writes the BSSID register and sets the network-type field of REG_CR to
 * managed/linked, preserving the TX/RX engine enables in that register's low
 * bytes. Mirrors rtw88's PORT_SET_BSSID + PORT_SET_NET_TYPE(RTW_NET_MGD_LINKED).
 */
pub(super) fn set_media_connected(regs: &Regs, bssid: &[u8; 6]) {
    let lo = u32::from_le_bytes([bssid[0], bssid[1], bssid[2], bssid[3]]);
    let hi = u16::from_le_bytes([bssid[4], bssid[5]]);
    regs.write32(REG_BSSID, lo);
    regs.write16(REG_BSSID + 4, hi);

    /*
     * Set the network type to managed/linked so the hardware accepts the access
     * point's unicast data frames. This alone gave the most data reception on
     * hardware; enabling the security engine here made the chip try to decrypt
     * every frame and drop what it could not (no key exists yet during the
     * handshake), so it is left off until keys are installed.
     */
    let cr = regs.read32(REG_CR);
    regs.write32(REG_CR, (cr & !NETTYPE_MASK) | (NET_TYPE_LINKED << NETTYPE_SHIFT));
}
