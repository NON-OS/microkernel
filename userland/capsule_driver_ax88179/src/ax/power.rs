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

//! The first steps of ax88179_reset: the PHY's power and the clocks, each
//! followed by the wait Linux gives it, then ax88179_auto_detach.

use nonos_libc::mk_idle_ms;
use nonos_usbnet::Bus;

use super::access::{read, read_mac, write_mac, Step};
use super::bits::{CLK_SELECT_ACS, CLK_SELECT_BCS, CLK_SELECT_ULR};
use super::bits::{PHYPWR_RSTCTL_AT, PHYPWR_RSTCTL_IPRL};
use super::regs::{ACCESS_EEPROM, AUTO_DETACH_ON, CLK_SELECT, EEPROM_AUTO_DETACH, PHYPWR_RSTCTL};

/// msleep(200) after the PHY comes out of reset, msleep(100) after the
/// clocks are selected (ax88179_reset).
const PHY_POWER_MS: u64 = 200;
const CLOCK_MS: u64 = 100;

pub(super) fn power_up<B: Bus>(bus: &mut B) -> Result<(), Step> {
    write_mac(bus, PHYPWR_RSTCTL, &0u16.to_le_bytes(), "AX_PHYPWR_RSTCTL power down refused")?;
    let on = PHYPWR_RSTCTL_IPRL.to_le_bytes();
    write_mac(bus, PHYPWR_RSTCTL, &on, "AX_PHYPWR_RSTCTL power up refused")?;
    mk_idle_ms(PHY_POWER_MS);
    let clocks = [CLK_SELECT_ACS | CLK_SELECT_BCS];
    write_mac(bus, CLK_SELECT, &clocks, "AX_CLK_SELECT write refused")?;
    mk_idle_ms(CLOCK_MS);
    Ok(())
}

/// When the EEPROM asks for it, the PHY detaches itself while unused.
/// Linux goes on without it when the EEPROM cannot be read, and so does
/// this; once it is asked for, each step that fails is named.
pub(super) fn auto_detach<B: Bus>(bus: &mut B) -> Result<(), Step> {
    let mut word = [0u8; 2];
    let at = (ACCESS_EEPROM, EEPROM_AUTO_DETACH, 1);
    if read(bus, at, &mut word, "EEPROM unread").is_err() {
        return Ok(());
    }
    let word = u16::from_le_bytes(word);
    if word == 0xffff || word & AUTO_DETACH_ON == 0 {
        return Ok(());
    }
    let mut clk = [0u8; 1];
    read_mac(bus, CLK_SELECT, &mut clk, "AX_CLK_SELECT unread")?;
    let detach = [clk[0] | CLK_SELECT_ULR];
    write_mac(bus, CLK_SELECT, &detach, "AX_CLK_SELECT auto detach refused")?;
    let mut pwr = [0u8; 2];
    read_mac(bus, PHYPWR_RSTCTL, &mut pwr, "AX_PHYPWR_RSTCTL unread")?;
    let pwr = (u16::from_le_bytes(pwr) | PHYPWR_RSTCTL_AT).to_le_bytes();
    write_mac(bus, PHYPWR_RSTCTL, &pwr, "AX_PHYPWR_RSTCTL auto detach refused")
}
