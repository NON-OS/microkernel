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

pub const VMWARE_VENDOR_ID: u16 = 0x15AD;
pub const PVSCSI_DEVICE_ID: u16 = 0x07C0;
/// The register window: a memory BAR of at least 32 KiB. VMware puts it in
/// BAR1 behind an I/O BAR0, QEMU in BAR0; the first memory BAR that large
/// is the one, as Linux takes it.
pub const MEM_SPACE: u64 = 0x8000;
