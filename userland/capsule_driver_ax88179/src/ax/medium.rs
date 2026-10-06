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

//! The medium mode and bulk IN row for the link the PHY resolved, as
//! ax88179_link_reset chooses them from GMII_PHY_PHYSR and the USB speed.

use super::bits::{MEDIUM_BASE, MEDIUM_EN_125MHZ, MEDIUM_FULL_DUPLEX, MEDIUM_GIGAMODE, MEDIUM_PS};
use super::bits::{USB_HS, USB_SS};
use super::bulkin::{fitted, BULKIN_SIZE};
use super::phy_regs::{PHYSR_100, PHYSR_FULL, PHYSR_GIGA, PHYSR_LINK, PHYSR_SMASK};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Medium {
    pub mode: u16,
    /// The AX_RX_BULKIN_QCTRL bytes, fitted to BULK_MAX.
    pub bulkin: [u8; 5],
}

/// `None` while the PHY reports no link. Jumbo mode is never set: the
/// stack's frames are at most 1514 bytes.
pub fn medium(physr: u16, link_sts: u8) -> Option<Medium> {
    if physr & PHYSR_LINK == 0 {
        return None;
    }
    let mut mode = MEDIUM_BASE;
    let row = match physr & PHYSR_SMASK {
        PHYSR_GIGA => {
            mode |= MEDIUM_GIGAMODE | MEDIUM_EN_125MHZ;
            if link_sts & USB_SS != 0 {
                0
            } else if link_sts & USB_HS != 0 {
                1
            } else {
                3
            }
        }
        PHYSR_100 => {
            mode |= MEDIUM_PS;
            if link_sts & (USB_SS | USB_HS) != 0 {
                2
            } else {
                3
            }
        }
        _ => 3,
    };
    if physr & PHYSR_FULL != 0 {
        mode |= MEDIUM_FULL_DUPLEX;
    }
    Some(Medium { mode, bulkin: fitted(BULKIN_SIZE[row]) })
}
