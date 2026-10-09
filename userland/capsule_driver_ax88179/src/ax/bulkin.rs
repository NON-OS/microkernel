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

//! Bulk IN aggregation (AX_RX_BULKIN_QCTRL): how many bytes of frames the
//! chip gathers into one bulk IN transfer before it sends it.

use nonos_usbnet::xhci::BULK_MAX;

/// AX88179_BULKIN_SIZE: ctrl, timer low, timer high, size, inter-frame
/// gap. Rows: gigabit on SuperSpeed, gigabit on high speed, 100 Mb/s on
/// either, and everything slower.
pub const BULKIN_SIZE: [[u8; 5]; 4] = [
    [7, 0x4f, 0, 0x12, 0xff],
    [7, 0x20, 3, 0x16, 0xff],
    [7, 0xae, 7, 0x18, 0xff],
    [7, 0xcc, 0x4c, 0x18, 8],
];

/// Linux sizes its receive buffer at 1024 * (size + 2) bytes for the size
/// byte it programs (ax88179_link_reset, rx_urb_size), 20 KiB and more for
/// its rows. driver.xhci0 moves at most BULK_MAX in one bulk IN, and the
/// same relation gives BULK_MAX for a size of 2.
pub const SIZE_FOR_BULK_MAX: u8 = (BULK_MAX / 1024 - 2) as u8;

/// A row with its size cut to SIZE_FOR_BULK_MAX. Its timer and gap stay,
/// so the chip still sends what it has gathered on Linux's timing.
pub fn fitted(row: [u8; 5]) -> [u8; 5] {
    let mut out = row;
    out[3] = out[3].min(SIZE_FOR_BULK_MAX);
    out
}
