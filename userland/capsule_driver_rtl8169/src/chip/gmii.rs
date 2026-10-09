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

/// RTL810x boards, the 10/100 parts, answer PCI id 0x8136. Linux marks that
/// id `RTL_CFG_NO_GBIT` (rtl8169_pci_tbl), and the XID lookup needs it: an
/// 8168gu or 8168h XID on such a board is the RTL8106eus or the RTL8107e.
const RTL810X_DEVICE_ID: u16 = 0x8136;

pub fn has_gmii(pci_device: u16) -> bool {
    pci_device != RTL810X_DEVICE_ID
}
