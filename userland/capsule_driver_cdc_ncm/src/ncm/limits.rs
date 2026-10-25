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

//! The NTB sizes in each direction (NCM 1.0, 6.2.1 and 6.2.7, as recalled,
//! unverified), held to what one bulk transfer of driver.xhci0 carries.

use nonos_usbnet::xhci::BULK_MAX;

/// The least NTB input size SET_NTB_INPUT_SIZE may give (NCM 1.0, 6.2.7).
const NTB_IN_MIN: usize = 2048;

/// The NTB input size asked of the device: no more than it offers or one
/// bulk transfer holds, and never under the 2048 the request allows.
pub fn rx_max(in_max: u32) -> usize {
    (in_max as usize).clamp(NTB_IN_MIN, BULK_MAX)
}

/// The longest NTB built before its padding byte: the device's
/// dwNtbOutMaxSize when one transfer holds it. When it does not, one byte
/// under a transfer, so a block that ends on a packet boundary below the
/// device's size still has room for the byte that ends it short (pad.rs).
pub fn tx_max(out_max: u32) -> usize {
    match out_max as usize {
        out if out <= BULK_MAX => out,
        _ => BULK_MAX - 1,
    }
}

/// A length that fills its last packet. A multiple of 64 counts too:
/// driver.xhci0 moves a high-speed or SuperSpeed pipe in 512 or 1024-byte
/// packets whatever the descriptor says, and both are multiples of 64.
pub fn ends_on_packet(len: usize, mps: usize) -> bool {
    len.is_multiple_of(64) || (mps != 0 && len.is_multiple_of(mps))
}
