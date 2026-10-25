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

pub(in super::super) const OP_HEALTHCHECK: u16 = 1;
pub(in super::super) const OP_CONTROLLER_INFO: u16 = 2;
pub(in super::super) const OP_PORT_LIST: u16 = 3;
pub(in super::super) const OP_CAPACITY: u16 = 4;
pub(in super::super) const OP_READ_BLOCKS: u16 = 5;
pub(in super::super) const OP_WRITE_BLOCKS: u16 = 6;
/// Before power goes, the kernel's alone (block_device::leave).
pub(in super::super) const OP_POWER_OFF: u16 = 9;
/// OP_POWER_OFF's flags bit for a restart: flush, no STANDBY IMMEDIATE.
pub(in super::super) const POWER_OFF_FLUSH_ONLY: u16 = 1 << 0;
pub(in super::super) const OP_FLUSH: u16 = 7;
