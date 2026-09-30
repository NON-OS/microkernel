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

use nonos_mac::apply;

use crate::constants::regs::{CFG9346_LOCK, CFG9346_UNLOCK, REG_CFG9346, REG_MAC0};
use crate::constants::MAC_LEN;
use crate::pio::Pio;

/// Draw a station address and program it into the IDR registers.
///
/// Replaces reading the factory address out of them: that address is unique to
/// the chip and every network the machine joins would log it. Fails closed, as
/// the other drivers do; the factory address is not a fallback.
pub fn program(pio: &Pio) -> Result<[u8; MAC_LEN], &'static str> {
    let mut mac = [0u8; MAC_LEN];
    let rc = nonos_libc::crypto_random(mac.as_mut_ptr(), MAC_LEN);
    if rc < 0 || (rc as usize) != MAC_LEN {
        return Err("rtl8139 no entropy for station address");
    }
    apply(&mut mac);

    // IDR takes writes only with the config lock open, as dwords, the way
    // 8139too's set_mac_address writes it; the lock closes on every path.
    pio.w8(REG_CFG9346, CFG9346_UNLOCK)?;
    let wrote = write_idr(pio, &mac);
    pio.w8(REG_CFG9346, CFG9346_LOCK)?;
    wrote?;

    let mut readback = [0u8; MAC_LEN];
    for (i, byte) in readback.iter_mut().enumerate() {
        *byte = pio.r8(REG_MAC0 + i as u16)?;
    }
    if readback != mac {
        return Err("rtl8139 station address did not take");
    }
    Ok(mac)
}

fn write_idr(pio: &Pio, mac: &[u8; MAC_LEN]) -> Result<(), &'static str> {
    pio.w32(REG_MAC0, u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]))?;
    pio.w32(REG_MAC0 + 4, mac[4] as u32 | (mac[5] as u32) << 8)
}
