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

pub const REALTEK_VENDOR_ID: u16 = 0x10EC;

/*
 * Realtek's own ids from Linux rtl8169_pci_tbl that this driver starts:
 * the PCI 8169/8110 (0x8169, 0x8167), the 8168/8111 (0x8161, 0x8162,
 * 0x8168, and the Killer E2500 0x2502 and E2600 0x2600 built on it), the
 * 10/100 RTL810x (0x8136, same descriptors and registers, RTL_CFG_NO_GBIT
 * in Linux) and the RTL8125 2.5 GbE (0x8125, Killer E3000 0x3000). The
 * chip behind an id is still identified from its XID before anything is
 * written. Left out: 0x8126 (RTL8126A, 5 GbE) and 0x8127 (RTL8127A,
 * 10 GbE), whose start differs from the 8125's; 0x8129, which 8139too
 * also claims; 0x5000 and 0x0e10, whose parts Linux does not name.
 */
pub const RTL8169_DEVICE_IDS: &[u16] =
    &[0x2502, 0x2600, 0x3000, 0x8125, 0x8136, 0x8161, 0x8162, 0x8167, 0x8168, 0x8169];
