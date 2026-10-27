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

//! The adapters this driver looks at: Realtek's own RTL8153 id and the
//! rebrands sold with the chip inside, each present in Linux v6.12's r8152
//! table. An id says only who sold the adapter; the chip's version register
//! (model.rs) says what is in it, and a chip other than an RTL8153 or
//! RTL8153B is refused by name there.

/// (idVendor, idProduct).
pub const ADAPTERS: [(u16, u16); 16] = [
    (0x0bda, 0x8153), // Realtek
    (0x17ef, 0x3062), // Lenovo
    (0x17ef, 0x3069),
    (0x17ef, 0x3082),
    (0x17ef, 0x7205),
    (0x17ef, 0x720c),
    (0x17ef, 0x7214),
    (0x17ef, 0x721e),
    (0x17ef, 0xa387),
    (0x045e, 0x07ab), // Microsoft
    (0x045e, 0x07c6),
    (0x045e, 0x0927),
    (0x2357, 0x0601), // TP-Link
    (0x13b1, 0x0041), // Linksys
    (0x04e8, 0xa101), // Samsung
    (0x0955, 0x09ff), // Nvidia
];

/// Whether the device descriptor's ids are on the list.
pub fn listed(vendor: u16, product: u16) -> bool {
    ADAPTERS.contains(&(vendor, product))
}
