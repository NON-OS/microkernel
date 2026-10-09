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

//! Devices with an ECM configuration that driver_rtl8153 serves in their
//! vendor mode instead, as Linux cdc_ether leaves them to r8152 (cdc_ether
//! products[], "Realtek RTL8153 Based USB 3.0 Ethernet Adapters" and the
//! Lenovo Powered USB-C Travel Hub).

const LEFT_TO_RTL8153: &[(u16, u16)] = &[(0x0bda, 0x8153), (0x17ef, 0x721e)];

pub fn left_to_vendor_driver(vendor: u16, product: u16) -> bool {
    LEFT_TO_RTL8153.contains(&(vendor, product))
}
