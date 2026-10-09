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

//! The devices Linux ax88179_178a binds, exactly its products[] table.
//! ASIX parts Linux gives to another driver (0b95:2790 and 0b95:2791,
//! which cdc_ether lists) are not here.

const PRODUCTS: &[(u16, u16)] = &[
    (0x0b95, 0x1790), // ASIX AX88179
    (0x0b95, 0x178a), // ASIX AX88178A
    (0x04b4, 0x3610), // Cypress GX3
    (0x2001, 0x4a00), // D-Link DUB-1312
    (0x0df6, 0x0072), // Sitecom USB 3.0 to Gigabit
    (0x04e8, 0xa100), // Samsung USB Ethernet Adapter
    (0x17ef, 0x304b), // Lenovo OneLinkDock
    (0x050d, 0x0128), // Belkin B2B128
    (0x0930, 0x0a13), // Toshiba USB 3.0 GBit
    (0x0711, 0x0179), // Magic Control Technology U3-A9003
    (0x07c9, 0x000e), // Allied Telesis AT-UMC2000
    (0x07c9, 0x000f), // Allied Telesis AT-UMC200
    (0x07c9, 0x0010), // Allied Telesis AT-UMC2000/SP
];

pub fn listed(vendor: u16, product: u16) -> bool {
    PRODUCTS.contains(&(vendor, product))
}
