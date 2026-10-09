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

use super::regs::REG_MAX_TX_PACKET_SIZE;
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Linux EarlySize: the 8111e-vl and later take frames in 128-byte units.
const EARLY_SIZE: u8 = 0x27;
/// Linux TxPacketMax: 8064 >> 7, the older 8168s and the 810x.
const TX_PACKET_MAX: u8 = (8064 >> 7) as u8;

/// Linux rtl_hw_start_8168, for the 8168 and 810x (VER_07 to VER_52).
pub fn start_8168(regs: &Regs, ver: MacVersion) {
    let size = if ver.is_8168evl_up() { EARLY_SIZE } else { TX_PACKET_MAX };
    // SAFETY: MaxTxPacketSize (0xEC) lies inside every mapped window.
    unsafe { regs.w8(REG_MAX_TX_PACKET_SIZE, size) };
}
