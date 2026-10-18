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

/// BusLogic, later Mylex.
pub const BUSLOGIC_VENDOR_ID: u16 = 0x104B;
/// The MultiMaster family (BT-946C, BT-948, BT-958): what VirtualBox and
/// VMware emulate. The FlashPoint (0x8130) speaks another interface.
pub const MULTIMASTER_DEVICE_ID: u16 = 0x1040;
/// The four registers in I/O BAR0.
pub const PORT_BAR: u8 = 0;
pub const PORTS: u16 = 4;
