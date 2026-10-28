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

//! The PCI ids the driver takes. Pure, so the kernel's proofs can hold its
//! inventory to them.

/// PCI vendor: Realtek.
pub const PCI_VENDOR_REALTEK: u16 = 0x10EC;

/// The RTL8821CE's device ids, as Linux v6.12 rtw88 binds them
/// (`rtw8821ce.c`, `rtw_8821ce_id_table`): 0xB821 and 0xC821.
pub const RTL8821CE_DEVICE_IDS: [u16; 2] = [0xB821, 0xC821];
