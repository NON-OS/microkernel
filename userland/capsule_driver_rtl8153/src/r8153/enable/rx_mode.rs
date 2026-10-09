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

//! What the chip accepts (Linux _rtl8152_set_rx_mode with every multicast
//! wanted, as for IFF_ALLMULTI): broadcast, frames to its own address,
//! and all multicast, the hash filter in PLA_MAR all ones. Not every
//! frame (RCR_AAP): the stack is not promiscuous.

use nonos_usbnet::Bus;

use crate::r8153::ocp::{set, update_dword, Dev, BYTE_EN_DWORD, PLA};
use crate::r8153::regs::bits::{RCR_AB, RCR_ACPT_ALL, RCR_AM, RCR_APM};
use crate::r8153::regs::pla::{MAR, RCR};

pub fn rx_mode<B: Bus>(dev: &mut Dev<B>) -> Result<(), i32> {
    // pla_ocp_write(PLA_MAR, BYTE_EN_DWORD, 8): both dwords whole, one
    // request.
    set(dev, PLA | BYTE_EN_DWORD, MAR, &[0xff; 8])?;
    update_dword(dev, PLA, RCR, RCR_ACPT_ALL, RCR_AB | RCR_APM | RCR_AM)
}
