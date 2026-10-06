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

use super::extended::REG_TX_CONFIG_V2;
use super::{lookup, lookup_extended, say, xid_of, Chip, ChipError, Lookup};
use crate::constants::regs::REG_TX_CONFIG;
use crate::regs::Regs;

/// The 8125 start writes Q_NUM_CTRL_8125 at 0x4800 (16 bits).
const WINDOW_8125: u64 = 0x4802;

/// Identify the chip as Linux rtl_init_one does, before anything is written,
/// and log one line either way. `window` is the length the broker mapped.
pub fn detect(regs: &Regs, window: u64, gmii: bool) -> Result<Chip, ChipError> {
    let found = identify(regs, window, gmii);
    match &found {
        Ok(chip) => say::found(chip),
        Err(e) => say::refused(e),
    }
    found
}

fn identify(regs: &Regs, window: u64, gmii: bool) -> Result<Chip, ChipError> {
    // SAFETY: TxConfig (0x40) lies inside every BAR this driver maps; the
    // broker grant is at least 0x100 bytes (setup::mmio).
    let txconfig = unsafe { regs.r32(REG_TX_CONFIG) };
    if txconfig == u32::MAX {
        return Err(ChipError::ReadFailed);
    }
    let xid = xid_of(txconfig);
    let chip = match lookup(xid, gmii) {
        Lookup::Known(chip) => chip,
        Lookup::Unknown => return Err(ChipError::Unknown { xid, extended: false }),
        Lookup::Extended => {
            if window < (REG_TX_CONFIG_V2 + 4) as u64 {
                return Err(ChipError::ExtendedOutOfWindow);
            }
            // SAFETY: the check above puts TX_CONFIG_V2 inside the window.
            let xid2 = unsafe { regs.r32(REG_TX_CONFIG_V2) };
            lookup_extended(xid2).ok_or(ChipError::Unknown { xid: xid2, extended: true })?
        }
    };
    // VER_70 and VER_80: rtl_hw_start_8126a and _8127a add PCIe Gen3 and
    // MAC steps of their own, and their PHYs report 5G and 10G.
    if chip.ver.0 >= 70 {
        return Err(ChipError::Unsupported(chip));
    }
    if chip.ver.is_8125() && window < WINDOW_8125 {
        return Err(ChipError::BarTooSmall(window));
    }
    Ok(chip)
}
